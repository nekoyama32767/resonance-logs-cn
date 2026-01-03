#pragma once
#include <cstdint>

namespace rust {
inline namespace cxxbridge1 {
class String;
template <typename T> class Vec;
}
}

namespace module_optimizer_ffi {

struct ModulePartFfi;
struct ModuleInfoFfi;
struct AttrBreakdownEntry;
struct ModuleSolutionFfi;
struct GpuSupportInfo;
struct ProgressInfoFfi;

::std::int32_t test_cuda_ffi();
::std::int32_t test_opencl_ffi();
GpuSupportInfo check_gpu_support_ffi();
ProgressInfoFfi get_progress_ffi();
void reset_progress_ffi();

::rust::Vec<ModuleSolutionFfi> strategy_enumeration_cpu_ffi(
    ::rust::Vec<ModuleInfoFfi> const& modules,
    ::rust::Vec<::std::int32_t> const& target_attributes,
    ::rust::Vec<::std::int32_t> const& exclude_attributes,
    ::rust::Vec<::std::int32_t> const& min_attr_ids,
    ::rust::Vec<::std::int32_t> const& min_attr_values,
    ::std::int32_t max_solutions,
    ::std::int32_t max_workers);

::rust::Vec<ModuleSolutionFfi> strategy_enumeration_gpu_ffi(
    ::rust::Vec<ModuleInfoFfi> const& modules,
    ::rust::Vec<::std::int32_t> const& target_attributes,
    ::rust::Vec<::std::int32_t> const& exclude_attributes,
    ::rust::Vec<::std::int32_t> const& min_attr_ids,
    ::rust::Vec<::std::int32_t> const& min_attr_values,
    ::std::int32_t max_solutions,
    ::std::int32_t max_workers);

::rust::Vec<ModuleSolutionFfi> optimize_modules_ffi(
    ::rust::Vec<ModuleInfoFfi> const& modules,
    ::rust::Vec<::std::int32_t> const& target_attributes,
    ::rust::Vec<::std::int32_t> const& exclude_attributes,
    ::std::int32_t max_solutions,
    ::std::int32_t max_attempts_multiplier,
    ::std::int32_t local_search_iterations);

}
