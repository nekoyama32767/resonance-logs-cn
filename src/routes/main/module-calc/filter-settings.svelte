<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";

  type AttrOption = { id: number; label: string };
  type MinReq = { attrId: number | null; value: number | null };

  let {
    attributeOptions = [],
    targetAttributes = $bindable<number[]>([]),
    excludeAttributes = $bindable<number[]>([]),
    minRequirements = $bindable<MinReq[]>([{ attrId: null, value: null }]),
  }: {
    attributeOptions: AttrOption[];
    targetAttributes: number[];
    excludeAttributes: number[];
    minRequirements: MinReq[];
  } = $props();

  function toggle(list: number[], id: number): number[] {
    return list.includes(id) ? list.filter((x) => x !== id) : [...list, id];
  }

  function updateMin(idx: number, field: "attrId" | "value", val: number | null) {
    const next = [...minRequirements];
    next[idx] = { ...next[idx], [field]: val };
    minRequirements = next;
  }

  function addMin() {
    minRequirements = [...minRequirements, { attrId: null, value: null }];
  }

  function removeMin(idx: number) {
    minRequirements = minRequirements.filter((_, i) => i !== idx);
  }
</script>

<div class="rounded-lg border border-border/60 bg-card/40 p-4 space-y-4">
  <div class="text-base font-semibold text-foreground">筛选设置</div>

  <div class="space-y-2">
    <div class="text-sm text-muted-foreground">目标属性, 选中后只会计算携带该属性的模组(模组数超过1000时可利用该设置先进行筛选)</div>
    <div class="flex flex-wrap gap-2">
      {#each attributeOptions as opt}
        <Button
          size="sm"
          variant={targetAttributes.includes(opt.id) ? "default" : "outline"}
          onclick={() => (targetAttributes = toggle(targetAttributes, opt.id))}
        >
          {opt.label}
        </Button>
      {/each}
    </div>
  </div>

  <div class="space-y-2">
    <div class="text-sm text-muted-foreground">排除属性</div>
    <div class="flex flex-wrap gap-2">
      {#each attributeOptions as opt}
        <Button
          size="sm"
          variant={excludeAttributes.includes(opt.id) ? "default" : "outline"}
          onclick={() => (excludeAttributes = toggle(excludeAttributes, opt.id))}
        >
          {opt.label}
        </Button>
      {/each}
    </div>
  </div>

  <div class="space-y-3">
    <div class="text-sm text-muted-foreground">最小属性要求</div>
    <div class="space-y-2">
      {#each minRequirements as req, idx}
        <div class="flex items-center gap-2">
          <select
            class="h-9 rounded-md border border-border bg-background px-2 text-sm"
            value={req.attrId ?? ""}
            on:change={(e) =>
              updateMin(idx, "attrId", Number((e.target as HTMLSelectElement).value) || null)}
          >
            <option value="">选择属性</option>
            {#each attributeOptions as opt}
              <option value={opt.id}>{opt.label}</option>
            {/each}
          </select>
          <Input
            type="number"
            min="0"
            class="w-24"
            value={req.value ?? ""}
            on:change={(e) =>
              updateMin(idx, "value", Number((e.target as HTMLInputElement).value) || null)}
          />
          <Button size="sm" variant="ghost" onclick={() => removeMin(idx)}>移除</Button>
        </div>
      {/each}
    </div>
    <Button size="sm" variant="outline" onclick={addMin}>+ 添加</Button>
  </div>
</div>

