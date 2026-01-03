use rand::prelude::*;
use rand::rngs::StdRng;
use resonance_logs_lib::module_optimizer::{
    ModuleInfo, ModulePart, OptimizeOptions, check_gpu_support, strategy_enumeration_cpu,
    strategy_enumeration_gpu,
};
use std::time::Instant;

fn generate_deterministic_modules(num_modules: i32, seed: u64) -> Vec<ModuleInfo> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut modules = Vec::new();

    let module_types = vec![0, 1, 2, 3, 4, 5];
    let attr_types = vec![1, 2, 3, 4, 5, 6, 7];

    for i in 0..num_modules {
        let config_id = *module_types.choose(&mut rng).unwrap();
        let num_attrs = rng.random_range(1..=3);

        let mut parts = Vec::new();
        for _ in 0..num_attrs {
            let attr_type = *attr_types.choose(&mut rng).unwrap();
            let value = rng.random_range(1..=6);
            parts.push(ModulePart {
                id: attr_type,
                name: format!("attr_{}", attr_type),
                value,
            });
        }

        modules.push(ModuleInfo {
            name: format!("test_module_{}", i),
            config_id,
            uuid: i,
            quality: rng.random_range(1..=5),
            parts,
        });
    }
    modules
}

#[test]
fn test_gpu_cpu_consistency() {
    let gpu_support = check_gpu_support();
    if !gpu_support.cuda_available && !gpu_support.opencl_available {
        println!("Skipping GPU consistency test: No GPU available");
        return;
    }

    let modules = generate_deterministic_modules(100, 42);

    let options = OptimizeOptions {
        max_solutions: 10,
        max_workers: 4,
        use_gpu: true,
        ..Default::default()
    };

    let cpu_results = strategy_enumeration_cpu(&modules, &options);
    let gpu_results = strategy_enumeration_gpu(&modules, &options);

    assert!(!cpu_results.is_empty(), "CPU should return results");
    assert!(!gpu_results.is_empty(), "GPU should return results");

    let cpu_best = cpu_results[0].score;
    let gpu_best = gpu_results[0].score;

    assert_eq!(cpu_best, gpu_best, "CPU and GPU best scores should match");
}

#[test]
#[ignore]
fn bench_large_scale_performance() {
    let gpu_support = check_gpu_support();
    if !gpu_support.cuda_available && !gpu_support.opencl_available {
        eprintln!("Skipping performance test: No GPU available");
        return;
    }

    let sizes = [500, 800];
    let options = OptimizeOptions {
        max_solutions: 100,
        max_workers: 24,
        use_gpu: true,
        ..Default::default()
    };

    println!("\n=== Performance Benchmark ===");
    for size in sizes {
        let modules = generate_deterministic_modules(size, 12345);

        let start = Instant::now();
        let _cpu = strategy_enumeration_cpu(&modules, &options);
        let cpu_time = start.elapsed();

        let start = Instant::now();
        let _gpu = strategy_enumeration_gpu(&modules, &options);
        let gpu_time = start.elapsed();

        println!(
            "Size: {:4} | CPU: {:>7.2?} | GPU: {:>7.2?} | Speedup: {:.2}x",
            size,
            cpu_time,
            gpu_time,
            cpu_time.as_secs_f64() / gpu_time.as_secs_f64()
        );
    }
    println!("=============================\n");
}
