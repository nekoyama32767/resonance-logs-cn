#include "resonance-logs-cn/src/module_optimizer/bridge.rs.h"
#include "module_optimizer.h"
#include <unordered_set>
#include <unordered_map>

#ifdef USE_CUDA
extern "C" int TestCuda();
#endif

#ifdef USE_OPENCL
extern "C" int TestOpenCL();
#endif

namespace module_optimizer_ffi {

static std::vector<ModuleInfo> convert_modules(::rust::Vec<ModuleInfoFfi> const& ffi_modules) {
    std::vector<ModuleInfo> modules;
    modules.reserve(ffi_modules.size());
    
    for (const auto& ffi_mod : ffi_modules) {
        std::vector<ModulePart> parts;
        parts.reserve(ffi_mod.parts.size());
        
        for (const auto& ffi_part : ffi_mod.parts) {
            parts.emplace_back(ffi_part.id, std::string(ffi_part.name), ffi_part.value);
        }
        
        modules.emplace_back(
            std::string(ffi_mod.name),
            ffi_mod.config_id,
            ffi_mod.uuid,
            ffi_mod.quality,
            parts
        );
    }
    
    return modules;
}

static ::rust::Vec<ModuleSolutionFfi> convert_solutions(const std::vector<ModuleSolution>& solutions) {
    ::rust::Vec<ModuleSolutionFfi> result;
    
    for (const auto& sol : solutions) {
        ModuleSolutionFfi ffi_sol;
        ffi_sol.score = sol.score;
        
        for (const auto& mod : sol.modules) {
            ModuleInfoFfi ffi_mod;
            ffi_mod.name = ::rust::String(mod.name);
            ffi_mod.config_id = mod.config_id;
            ffi_mod.uuid = mod.uuid;
            ffi_mod.quality = mod.quality;
            
            for (const auto& part : mod.parts) {
                ModulePartFfi ffi_part;
                ffi_part.id = part.id;
                ffi_part.name = ::rust::String(part.name);
                ffi_part.value = part.value;
                ffi_mod.parts.push_back(ffi_part);
            }
            
            ffi_sol.modules.push_back(ffi_mod);
        }
        
        for (const auto& [name, value] : sol.attr_breakdown) {
            AttrBreakdownEntry entry;
            entry.name = ::rust::String(name);
            entry.value = value;
            ffi_sol.attr_breakdown.push_back(entry);
        }
        
        result.push_back(ffi_sol);
    }
    
    return result;
}

static std::unordered_set<int> to_set(::rust::Vec<::std::int32_t> const& vec) {
    std::unordered_set<int> result;
    for (const auto& v : vec) {
        result.insert(v);
    }
    return result;
}

static std::unordered_map<int, int> to_map(
    ::rust::Vec<::std::int32_t> const& ids, 
    ::rust::Vec<::std::int32_t> const& values) {
    std::unordered_map<int, int> result;
    for (size_t i = 0; i < ids.size() && i < values.size(); ++i) {
        result[ids[i]] = values[i];
    }
    return result;
}

::std::int32_t test_cuda_ffi() {
#ifdef USE_CUDA
    return ::TestCuda();
#else
    return 0;
#endif
}

::std::int32_t test_opencl_ffi() {
#ifdef USE_OPENCL
    return ::TestOpenCL();
#else
    return 0;
#endif
}

GpuSupportInfo check_gpu_support_ffi() {
    GpuSupportInfo info;
    info.cuda_available = test_cuda_ffi() == 1;
    info.opencl_available = test_opencl_ffi() == 1;
    return info;
}

::rust::Vec<ModuleSolutionFfi> strategy_enumeration_cpu_ffi(
    ::rust::Vec<ModuleInfoFfi> const& modules,
    ::rust::Vec<::std::int32_t> const& target_attributes,
    ::rust::Vec<::std::int32_t> const& exclude_attributes,
    ::rust::Vec<::std::int32_t> const& min_attr_ids,
    ::rust::Vec<::std::int32_t> const& min_attr_values,
    ::std::int32_t max_solutions,
    ::std::int32_t max_workers) {
    
    auto cpp_modules = convert_modules(modules);
    auto result = ModuleOptimizerCpp::StrategyEnumeration(
        cpp_modules,
        to_set(target_attributes),
        to_set(exclude_attributes),
        to_map(min_attr_ids, min_attr_values),
        max_solutions,
        max_workers
    );
    
    return convert_solutions(result);
}

::rust::Vec<ModuleSolutionFfi> strategy_enumeration_gpu_ffi(
    ::rust::Vec<ModuleInfoFfi> const& modules,
    ::rust::Vec<::std::int32_t> const& target_attributes,
    ::rust::Vec<::std::int32_t> const& exclude_attributes,
    ::rust::Vec<::std::int32_t> const& min_attr_ids,
    ::rust::Vec<::std::int32_t> const& min_attr_values,
    ::std::int32_t max_solutions,
    ::std::int32_t max_workers) {
    
    auto cpp_modules = convert_modules(modules);
    auto result = ModuleOptimizerCpp::StrategyEnumerationGPU(
        cpp_modules,
        to_set(target_attributes),
        to_set(exclude_attributes),
        to_map(min_attr_ids, min_attr_values),
        max_solutions,
        max_workers
    );
    
    return convert_solutions(result);
}

::rust::Vec<ModuleSolutionFfi> optimize_modules_ffi(
    ::rust::Vec<ModuleInfoFfi> const& modules,
    ::rust::Vec<::std::int32_t> const& target_attributes,
    ::rust::Vec<::std::int32_t> const& exclude_attributes,
    ::std::int32_t max_solutions,
    ::std::int32_t max_attempts_multiplier,
    ::std::int32_t local_search_iterations) {
    
    auto cpp_modules = convert_modules(modules);
    auto result = ModuleOptimizerCpp::OptimizeModules(
        cpp_modules,
        to_set(target_attributes),
        to_set(exclude_attributes),
        max_solutions,
        max_attempts_multiplier,
        local_search_iterations
    );
    
    return convert_solutions(result);
}

ProgressInfoFfi get_progress_ffi() {
    auto p = ModuleOptimizerCpp::GetProgress();
    ProgressInfoFfi info;
    info.processed = p.first;
    info.total = p.second;
    return info;
}

void reset_progress_ffi() {
    ModuleOptimizerCpp::ResetProgress();
}

}
