use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Read version from tauri.conf.json and expose as APP_VERSION environment variable
    let tauri_conf =
        std::fs::read_to_string("tauri.conf.json").expect("Failed to read tauri.conf.json");
    let conf: serde_json::Value =
        serde_json::from_str(&tauri_conf).expect("Failed to parse tauri.conf.json");
    let version = conf["version"]
        .as_str()
        .expect("No version field in tauri.conf.json");
    println!("cargo:rustc-env=APP_VERSION={}", version);

    // Forward API URLs from build environment to compile-time env vars
    if let Ok(url) = env::var("UPLOAD_API_URL") {
        println!("cargo:rustc-env=UPLOAD_API_URL={}", url);
    }
    if let Ok(url) = env::var("TRACKING_API_URL") {
        println!("cargo:rustc-env=TRACKING_API_URL={}", url);
    }

    // Build module_optimizer C++ code
    build_module_optimizer();

    // Use the standard debug_assertions cfg to differentiate dev vs release.
    if cfg!(debug_assertions) {
        println!("DEBUG (dev) BUILD");
        tauri_build::build();
    } else {
        let mut windows = tauri_build::WindowsAttributes::new();
        windows = windows.app_manifest(include_str!("app.manifest"));
        tauri_build::try_build(tauri_build::Attributes::new().windows_attributes(windows))
            .expect("failed to run build script");
    }
}

fn build_module_optimizer() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let cpp_dir = manifest_dir.join("src/module_optimizer/cpp");

    // Check if cpp directory exists
    if !cpp_dir.exists() {
        println!(
            "cargo:warning=C++ directory not found: {:?}, skipping module_optimizer build",
            cpp_dir
        );
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Detect CUDA and OpenCL
    let use_cuda = detect_cuda();
    let use_opencl = detect_opencl();

    println!("cargo:warning=CUDA detected: {}", use_cuda);
    println!("cargo:warning=OpenCL detected: {}", use_opencl);

    // Build C++ source files list
    let mut cpp_sources = vec![
        cpp_dir.join("module_optimizer.cpp"),
        cpp_dir.join("ffi_bridge.cpp"),
    ];

    // Compile CUDA code if available
    let cuda_obj = if use_cuda {
        compile_cuda(&cpp_dir, &out_dir)
    } else {
        None
    };

    let cuda_enabled = cuda_obj.is_some();
    if cuda_enabled {
        println!("cargo:rustc-cfg=feature=\"cuda\"");
    }

    // Add OpenCL source file if available
    if use_opencl {
        println!("cargo:rustc-cfg=feature=\"opencl\"");
        cpp_sources.push(cpp_dir.join("module_optimizer_opencl.cpp"));
    }

    // Use cxx-build to compile
    let mut build = cxx_build::bridge("src/module_optimizer/bridge.rs");

    build
        .files(&cpp_sources)
        .include(&cpp_dir)
        .std("c++17")
        .flag_if_supported("/utf-8")
        .flag_if_supported("/EHsc")
        .flag_if_supported("/bigobj")
        .flag_if_supported("/MD")
        .flag_if_supported("-O2");

    // CUDA configuration
    if cuda_enabled {
        if let Some(cuda_home) = find_cuda_home() {
            println!("cargo:warning=Linking CUDA from: {}", cuda_home.display());
            
            build
                .define("USE_CUDA", None)
                .include(cuda_home.join("include"));

            println!(
                "cargo:rustc-link-search=native={}",
                cuda_home.join("lib/x64").display()
            );
            println!("cargo:rustc-link-lib=cudart_static");
            println!("cargo:rustc-link-lib=cuda");

            if let Some(ref obj) = cuda_obj {
                build.object(obj);
            }
        }
    }

    // OpenCL configuration
    if use_opencl {
        build.define("USE_OPENCL", None);

        if let Some(opencl_path) = find_opencl() {
            build.include(opencl_path.join("include"));
            println!(
                "cargo:rustc-link-search={}",
                opencl_path.join("lib/x64").display()
            );
        }
        println!("cargo:rustc-link-lib=OpenCL");
    }

    build.compile("module_optimizer_cpp");

    // Rerun if changed
    println!("cargo:rerun-if-changed=src/module_optimizer/bridge.rs");
    println!("cargo:rerun-if-changed=src/module_optimizer/cpp/");
}

fn detect_cuda() -> bool {
    Command::new("nvcc")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn detect_opencl() -> bool {
    find_opencl().is_some()
}

fn find_cuda_home() -> Option<PathBuf> {
    // Try multiple possible paths
    let paths = [
        env::var("CUDA_HOME").ok(),
        env::var("CUDA_PATH").ok(),
        Some(r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.9".to_string()),
    ];

    for path in paths.into_iter().flatten() {
        let p = PathBuf::from(&path);
        if p.exists() {
            return Some(p);
        }
    }
    None
}

fn find_opencl() -> Option<PathBuf> {
    // Check CUDA path for OpenCL (NVIDIA ships OpenCL with CUDA)
    if let Some(cuda_home) = find_cuda_home() {
        let opencl_lib = cuda_home.join("lib/x64/OpenCL.lib");
        if opencl_lib.exists() {
            return Some(cuda_home);
        }
    }

    // Check OPENCL_HOME environment variable
    if let Ok(opencl_home) = env::var("OPENCL_HOME") {
        let p = PathBuf::from(&opencl_home);
        if p.join("lib/x64/OpenCL.lib").exists() {
            return Some(p);
        }
    }

    None
}

fn compile_cuda(cpp_dir: &PathBuf, out_dir: &PathBuf) -> Option<PathBuf> {
    let cuda_home = find_cuda_home()?;
    let cuda_file = cpp_dir.join("module_optimizer_cuda.cu");
    let obj_file = out_dir.join("module_optimizer_cuda.obj");

    if !cuda_file.exists() {
        println!(
            "cargo:warning=CUDA source file not found: {:?}",
            cuda_file
        );
        return None;
    }

    // Find Visual Studio
    let vs_paths = [
        r"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat",
    ];

    let vs_vars = vs_paths.iter().find(|p| PathBuf::from(p).exists())?;

    // Create a temporary batch file to handle environment setup and compilation
    let bat_file = out_dir.join("compile_cuda.bat");
    let nvcc_cmd = format!(
        r#"nvcc -c "{}" -o "{}" -std=c++17 --compiler-options "/O2,/std:c++17,/EHsc,/MD" --use_fast_math -I"{}" -gencode=arch=compute_75,code=sm_75 -gencode=arch=compute_86,code=sm_86 -gencode=arch=compute_89,code=sm_89 -gencode=arch=compute_120,code=sm_120"#,
        cuda_file.display(),
        obj_file.display(),
        cuda_home.join("include").display()
    );

    let bat_content = format!(
        "@echo off\r\ncall \"{}\"\r\nif %errorlevel% neq 0 exit /b %errorlevel%\r\n{}\r\n",
        vs_vars, nvcc_cmd
    );

    std::fs::write(&bat_file, bat_content).expect("Failed to write compile_cuda.bat");

    println!("cargo:warning=Compiling CUDA using batch file: {}", bat_file.display());

    let output = Command::new("cmd")
        .args(["/C", bat_file.to_str().unwrap()])
        .output()
        .ok()?;

    if output.status.success() {
        println!("cargo:warning=CUDA compilation successful");
        Some(obj_file)
    } else {
        println!("cargo:warning=CUDA compilation failed, falling back to CPU version");
        println!("cargo:warning=CUDA output: {}", String::from_utf8_lossy(&output.stdout));
        println!("cargo:warning=CUDA error: {}", String::from_utf8_lossy(&output.stderr));
        None
    }
}
