<script lang="ts">
  let { moduleCount = null }: { moduleCount: number | null } = $props();
</script>

<div class="rounded-lg border border-border/60 bg-card/40 p-4 space-y-1">
  <div class="text-base font-semibold text-foreground">数据状态</div>
  <div class="text-sm text-muted-foreground">
    模组数量：{moduleCount ?? "未同步"}
  </div>
</div>

