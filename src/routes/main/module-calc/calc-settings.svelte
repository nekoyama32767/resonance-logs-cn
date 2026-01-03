<script lang="ts">
  import { Switch } from "$lib/components/ui/switch";

  let {
    useGpu = $bindable(true),
    gpuSupport = $bindable<{ cuda_available: boolean; opencl_available: boolean } | null>(null),
  } = $props();
</script>

<div class="rounded-lg border border-border/60 bg-card/40 p-4 space-y-3">
  <div class="text-base font-semibold text-foreground">计算设置</div>
  <div class="flex items-center gap-3">
    <Switch bind:checked={useGpu} />
    <div class="text-sm text-foreground">GPU 加速</div>
    {#if gpuSupport}
      <div class="text-xs text-muted-foreground">
        CUDA: {gpuSupport.cuda_available ? "可用" : "不可用"} · OpenCL: {gpuSupport.opencl_available ? "可用" : "不可用"}
      </div>
    {/if}
  </div>
</div>

