<script lang="ts">
  import { onMount } from "svelte";
  import { commands, type BuffDefinition, type BuffNameInfo } from "$lib/bindings";
  import SettingsSwitch from "../dps/settings/settings-switch.svelte";
  import {
    createDefaultSkillMonitorProfile,
    SETTINGS,
    type SkillMonitorProfile,
  } from "$lib/settings-store";
  import {
    findAnySkillByBaseId,
    findResonanceSkill,
    getClassConfigs,
    getSkillsByClass,
    searchResonanceSkills,
  } from "$lib/skill-mappings";

  let availableBuffs = $state<BuffDefinition[]>([]);
  let buffNames = $state(new Map<number, BuffNameInfo>());
  let buffSearch = $state("");
  let buffSearchResults = $state<BuffNameInfo[]>([]);
  let resonanceSearch = $state("");
  onMount(() => {
    void (async () => {
      const res = await commands.getAvailableBuffs();
      if (res.status === "ok") {
        availableBuffs = res.data;
      }
    })();
  });

  const classConfigs = $derived(getClassConfigs());
  const profiles = $derived(SETTINGS.skillMonitor.state.profiles);
  const activeProfileIndex = $derived(
    Math.min(
      Math.max(SETTINGS.skillMonitor.state.activeProfileIndex, 0),
      Math.max(0, profiles.length - 1),
    ),
  );
  const activeProfile = $derived(
    profiles[activeProfileIndex] ?? createDefaultSkillMonitorProfile(),
  );
  const selectedClassKey = $derived(activeProfile.selectedClass);
  const classSkills = $derived(getSkillsByClass(selectedClassKey));
  const monitoredSkillIds = $derived(activeProfile.monitoredSkillIds);
  const monitoredBuffIds = $derived(activeProfile.monitoredBuffIds);
  const showSkillCdGroup = $derived(
    activeProfile.overlayVisibility?.showSkillCdGroup ?? true,
  );
  const showResourceGroup = $derived(
    activeProfile.overlayVisibility?.showResourceGroup ?? true,
  );

  function updateActiveProfile(
    updater: (profile: SkillMonitorProfile) => SkillMonitorProfile,
  ) {
    const state = SETTINGS.skillMonitor.state;
    const currentProfiles = state.profiles;
    if (currentProfiles.length === 0) {
      state.profiles = [createDefaultSkillMonitorProfile()];
      state.activeProfileIndex = 0;
      return;
    }

    const index = Math.min(
      Math.max(state.activeProfileIndex, 0),
      currentProfiles.length - 1,
    );
    state.profiles = currentProfiles.map((profile, i) =>
      i === index ? updater(profile) : profile,
    );
  }

  function setActiveProfileIndex(index: number) {
    const maxIndex = Math.max(0, SETTINGS.skillMonitor.state.profiles.length - 1);
    SETTINGS.skillMonitor.state.activeProfileIndex = Math.min(
      Math.max(index, 0),
      maxIndex,
    );
  }

  function addProfile() {
    const nextIndex = SETTINGS.skillMonitor.state.profiles.length + 1;
    const nextProfile = createDefaultSkillMonitorProfile(`方案 ${nextIndex}`);
    SETTINGS.skillMonitor.state.profiles = [
      ...SETTINGS.skillMonitor.state.profiles,
      nextProfile,
    ];
    SETTINGS.skillMonitor.state.activeProfileIndex =
      SETTINGS.skillMonitor.state.profiles.length - 1;
  }

  function renameActiveProfile() {
    const nextName = window.prompt("请输入新的方案名称", activeProfile.name);
    if (!nextName) return;
    const trimmedName = nextName.trim();
    if (!trimmedName) return;
    updateActiveProfile((profile) => ({ ...profile, name: trimmedName }));
  }

  function removeActiveProfile() {
    const state = SETTINGS.skillMonitor.state;
    if (state.profiles.length <= 1) return;
    const index = Math.min(
      Math.max(state.activeProfileIndex, 0),
      state.profiles.length - 1,
    );
    state.profiles = state.profiles.filter((_, i) => i !== index);
    state.activeProfileIndex = Math.min(index, state.profiles.length - 1);
  }

  function setSelectedClass(classKey: string) {
    updateActiveProfile((profile) => ({
      ...profile,
      selectedClass: classKey,
      monitoredSkillIds: [],
    }));
  }

  function toggleSkill(skillId: number) {
    const current = monitoredSkillIds;
    const exists = current.includes(skillId);
    if (exists) {
      updateActiveProfile((profile) => ({
        ...profile,
        monitoredSkillIds: current.filter((id) => id !== skillId),
      }));
      return;
    }
    if (current.length >= 10) return;
    updateActiveProfile((profile) => ({
      ...profile,
      monitoredSkillIds: [...current, skillId],
    }));
  }

  function isSelected(skillId: number): boolean {
    return monitoredSkillIds.includes(skillId);
  }

  const filteredResonanceSkills = $derived.by(() =>
    searchResonanceSkills(resonanceSearch),
  );
  const selectedResonanceSkills = $derived.by(
    () =>
      monitoredSkillIds
        .map((id) => findResonanceSkill(id))
        .filter((skill): skill is NonNullable<typeof skill> => Boolean(skill))
        .slice(0, 10),
  );

  function clearSkills() {
    updateActiveProfile((profile) => ({ ...profile, monitoredSkillIds: [] }));
  }

  function clearBuffs() {
    updateActiveProfile((profile) => ({ ...profile, monitoredBuffIds: [] }));
  }

  function toggleBuff(buffId: number) {
    const current = monitoredBuffIds;
    const exists = current.includes(buffId);
    if (exists) {
      updateActiveProfile((profile) => ({
        ...profile,
        monitoredBuffIds: current.filter((id) => id !== buffId),
      }));
      return;
    }
    updateActiveProfile((profile) => ({
      ...profile,
      monitoredBuffIds: [...current, buffId],
    }));
  }

  function isBuffSelected(buffId: number): boolean {
    return monitoredBuffIds.includes(buffId);
  }

  const filteredBuffs = $derived.by(() => {
    const ids = new Set<number>();
    const merged: BuffNameInfo[] = [];
    for (const item of buffSearchResults) {
      if (ids.has(item.baseId)) continue;
      ids.add(item.baseId);
      merged.push(item);
    }
    return merged;
  });
  const availableBuffMap = $derived.by(() => {
    const map = new Map<number, BuffDefinition>();
    for (const buff of availableBuffs) {
      map.set(buff.baseId, buff);
    }
    return map;
  });
  const selectedBuffs = $derived.by(
    () =>
      monitoredBuffIds
        .map((id) => availableBuffs.find((buff) => buff.baseId === id))
        .filter(Boolean) as BuffDefinition[],
  );

  $effect(() => {
    const ids = monitoredBuffIds;
    if (ids.length === 0) return;
    void (async () => {
      const missing = ids.filter((id) => !buffNames.has(id));
      if (missing.length === 0) return;
      const res = await commands.getBuffNames(missing);
      if (res.status !== "ok") return;
      const next = new Map(buffNames);
      for (const item of res.data) {
        next.set(item.baseId, item);
      }
      buffNames = next;
    })();
  });

  $effect(() => {
    const keyword = buffSearch.trim();
    if (!keyword) {
      buffSearchResults = [];
      return;
    }
    void (async () => {
      const res = await commands.searchBuffsByName(keyword, 120);
      if (res.status !== "ok") return;
      buffSearchResults = res.data;
    })();
  });

  function setOverlaySectionVisibility(
    key: "showSkillCdGroup" | "showResourceGroup",
    checked: boolean,
  ) {
    updateActiveProfile((profile) => ({
      ...profile,
      overlayVisibility: {
        showSkillCdGroup: profile.overlayVisibility?.showSkillCdGroup ?? true,
        showResourceGroup: profile.overlayVisibility?.showResourceGroup ?? true,
        [key]: checked,
      },
    }));
  }

  function toggleOverlaySectionVisibility(
    key: "showSkillCdGroup" | "showResourceGroup",
  ) {
    const current =
      key === "showSkillCdGroup" ? showSkillCdGroup : showResourceGroup;
    setOverlaySectionVisibility(key, !current);
  }
</script>

<div class="space-y-6">
  <div class="rounded-lg border border-border/60 bg-card/40 p-4 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.02)] space-y-2">
    <SettingsSwitch
      bind:checked={SETTINGS.skillMonitor.state.enabled}
      label="启用技能监控"
      description="开启后将实时推送技能CD数据到悬浮窗口"
    />
  </div>

  <div class="rounded-lg border border-border/60 bg-card/40 p-4 space-y-4 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.02)]">
    <div>
      <h2 class="text-base font-semibold text-foreground">配置方案</h2>
      <p class="text-xs text-muted-foreground">
        可创建多个角色监控方案并快速切换
      </p>
    </div>
    <div class="flex flex-wrap items-center gap-2">
      <select
        class="w-full sm:w-72 rounded border border-border/60 bg-muted/30 px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/50"
        value={activeProfileIndex}
        onchange={(event) =>
          setActiveProfileIndex(Number((event.currentTarget as HTMLSelectElement).value))}
      >
        {#each profiles as profile, idx (idx)}
          <option value={idx}>{profile.name}</option>
        {/each}
      </select>
      <button
        type="button"
        class="text-xs px-3 py-2 rounded border border-border/60 text-foreground hover:bg-muted/40 transition-colors"
        onclick={addProfile}
      >
        新建方案
      </button>
      <button
        type="button"
        class="text-xs px-3 py-2 rounded border border-border/60 text-foreground hover:bg-muted/40 transition-colors"
        onclick={renameActiveProfile}
      >
        重命名
      </button>
      <button
        type="button"
        class="text-xs px-3 py-2 rounded border border-border/60 text-destructive hover:bg-destructive/10 transition-colors disabled:text-muted-foreground disabled:hover:bg-transparent"
        onclick={removeActiveProfile}
        disabled={profiles.length <= 1}
      >
        删除方案
      </button>
    </div>
  </div>

  <div class="rounded-lg border border-border/60 bg-card/40 p-4 space-y-4 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.02)]">
    <div>
      <h2 class="text-base font-semibold text-foreground">Overlay 区域显示</h2>
      <p class="text-xs text-muted-foreground">
        可分别控制技能区和资源区是否显示（按方案保存）
      </p>
    </div>
    <div class="space-y-2">
      <div class="flex flex-wrap gap-2">
        <button
          type="button"
          class="px-3 py-2 rounded-lg text-sm font-medium border transition-colors {showSkillCdGroup
            ? 'bg-primary text-primary-foreground border-primary'
            : 'bg-muted/30 text-foreground border-border/60 hover:bg-muted/50'}"
          onclick={() => toggleOverlaySectionVisibility("showSkillCdGroup")}
        >
          技能CD区：{showSkillCdGroup ? "显示" : "隐藏"}
        </button>
        <button
          type="button"
          class="px-3 py-2 rounded-lg text-sm font-medium border transition-colors {showResourceGroup
            ? 'bg-primary text-primary-foreground border-primary'
            : 'bg-muted/30 text-foreground border-border/60 hover:bg-muted/50'}"
          onclick={() => toggleOverlaySectionVisibility("showResourceGroup")}
        >
          资源监控区：{showResourceGroup ? "显示" : "隐藏"}
        </button>
      </div>
      <p class="text-xs text-muted-foreground">
        点击按钮切换显示状态（按方案保存）
      </p>
    </div>
  </div>

  <div class="rounded-lg border border-border/60 bg-card/40 p-4 space-y-4 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.02)]">
    <div>
      <h2 class="text-base font-semibold text-foreground">职业选择</h2>
      <p class="text-xs text-muted-foreground">
        支持青岚骑士、冰法职业
      </p>
    </div>
    <div class="flex flex-wrap gap-2">
      {#each classConfigs as config (config.classKey)}
        <button
          type="button"
          class="px-3 py-2 rounded-lg text-sm font-medium border transition-colors {selectedClassKey === config.classKey
            ? 'bg-primary text-primary-foreground border-primary'
            : 'bg-muted/30 text-foreground border-border/60 hover:bg-muted/50'}"
          onclick={() => setSelectedClass(config.classKey)}
        >
          {config.className}
        </button>
      {/each}
    </div>
  </div>

  <div class="rounded-lg border border-border/60 bg-card/40 p-4 space-y-4 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.02)]">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-base font-semibold text-foreground">技能选择</h2>
        <p class="text-xs text-muted-foreground">
          最多监控 10 个技能（2行 x 5列）
        </p>
      </div>
      <div class="flex items-center gap-3">
        <div class="text-xs text-muted-foreground">
          已选 {monitoredSkillIds.length}/10
        </div>
        <button
          type="button"
          class="text-xs px-2 py-1 rounded border border-border/60 text-muted-foreground hover:text-foreground hover:bg-muted/40 transition-colors"
          onclick={clearSkills}
        >
          清空
        </button>
      </div>
    </div>

    <div class="grid grid-cols-[repeat(auto-fill,minmax(56px,1fr))] gap-3">
      {#each classSkills as skill (skill.skillId)}
        <button
          type="button"
          class="relative group rounded-lg border overflow-hidden transition-colors {isSelected(skill.skillId)
            ? 'border-primary ring-1 ring-primary'
            : 'border-border/60 hover:border-border'}"
          onclick={() => toggleSkill(skill.skillId)}
        >
          {#if skill.imagePath}
            <img
              src={skill.imagePath}
              alt={skill.name}
              class="w-full h-full object-cover aspect-square"
            />
          {:else}
            <div class="w-full h-full aspect-square flex items-center justify-center bg-muted/30 text-xs text-muted-foreground">
              未配置
            </div>
          {/if}
          <div class="absolute inset-x-0 bottom-0 bg-black/50 text-[10px] text-white px-1 py-0.5 truncate">
            {skill.name || `#${skill.skillId}`}
          </div>
        </button>
      {/each}
    </div>
  </div>

  <div class="rounded-lg border border-border/60 bg-card/40 p-4 space-y-4 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.02)]">
    <div class="flex items-center justify-between gap-3">
      <div>
        <h2 class="text-base font-semibold text-foreground">共鸣技能</h2>
        <p class="text-xs text-muted-foreground">
          通过搜索选择共鸣技能，与普通技能共享 10 个监控格
        </p>
      </div>
      <div class="text-xs text-muted-foreground">
        已选 {selectedResonanceSkills.length}
      </div>
    </div>

    <input
      class="w-full sm:w-64 rounded border border-border/60 bg-muted/30 px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/50"
      placeholder="搜索共鸣技能名称"
      bind:value={resonanceSearch}
    />

    {#if resonanceSearch.trim().length > 0}
      <div class="grid grid-cols-[repeat(auto-fill,minmax(56px,1fr))] gap-3">
        {#each filteredResonanceSkills as skill (skill.skillId)}
          <button
            type="button"
            class="relative group rounded-lg border overflow-hidden transition-colors {isSelected(skill.skillId)
              ? 'border-primary ring-1 ring-primary'
              : 'border-border/60 hover:border-border'}"
            title={skill.name}
            onclick={() => toggleSkill(skill.skillId)}
          >
            <img
              src={skill.imagePath}
              alt={skill.name}
              class="w-full h-full object-contain aspect-square bg-muted/20"
            />
            <div class="absolute inset-x-0 bottom-0 bg-black/50 text-[10px] text-white px-1 py-0.5 truncate">
              {skill.name}
            </div>
          </button>
        {/each}
      </div>
    {:else}
      <div class="text-xs text-muted-foreground">请输入关键词搜索共鸣技能</div>
    {/if}

    <div class="space-y-2">
      <div class="text-xs text-muted-foreground">已选共鸣技能</div>
      <div class="flex flex-wrap gap-2">
        {#each selectedResonanceSkills as skill (skill.skillId)}
          <button
            type="button"
            class="relative rounded-md border border-border/60 overflow-hidden bg-muted/20 size-12 hover:border-border hover:bg-muted/30"
            title={skill.name}
            onclick={() => toggleSkill(skill.skillId)}
          >
            <img
              src={skill.imagePath}
              alt={skill.name}
              class="w-full h-full object-contain"
            />
            <div class="absolute inset-x-0 bottom-0 bg-black/60 text-[9px] text-white px-1 py-0.5 truncate">
              {skill.name}
            </div>
          </button>
        {/each}
        {#if selectedResonanceSkills.length === 0}
          <div class="text-xs text-muted-foreground">未选择共鸣技能</div>
        {/if}
      </div>
    </div>
  </div>

  <div class="rounded-lg border border-border/60 bg-card/40 p-4 space-y-4 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.02)]">
    <div class="flex items-center justify-between gap-3">
      <div>
        <h2 class="text-base font-semibold text-foreground">Buff 监控</h2>
        <p class="text-xs text-muted-foreground">
          统一通过 Buff 名称搜索（含有图标/无图标 Buff）
        </p>
      </div>
      <div class="flex items-center gap-3">
        <div class="text-xs text-muted-foreground">
          已选 {monitoredBuffIds.length}
        </div>
        <button
          type="button"
          class="text-xs px-2 py-1 rounded border border-border/60 text-muted-foreground hover:text-foreground hover:bg-muted/40 transition-colors"
          onclick={clearBuffs}
        >
          清空
        </button>
      </div>
    </div>

    <div class="flex flex-wrap gap-3 items-center">
      <input
        class="w-full sm:w-64 rounded border border-border/60 bg-muted/30 px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-primary/50"
        placeholder="搜索 Buff 名称"
        bind:value={buffSearch}
      />
    </div>

  {#if buffSearch.trim().length > 0}
    <div class="grid grid-cols-[repeat(auto-fill,minmax(56px,1fr))] gap-3">
      {#each filteredBuffs as buff (buff.baseId)}
        {@const iconBuff = availableBuffMap.get(buff.baseId)}
        <button
          type="button"
          class="relative group rounded-lg border overflow-hidden transition-colors {isBuffSelected(buff.baseId)
            ? 'border-primary ring-1 ring-primary'
            : 'border-border/60 hover:border-border'}"
          title={buff.name}
          onclick={() => toggleBuff(buff.baseId)}
        >
          {#if iconBuff}
            <img
              src={iconBuff.talentSpriteFile
                ? `/images/talent/${iconBuff.talentSpriteFile}`
                : `/images/buff/${iconBuff.spriteFile}`}
              alt={iconBuff.name}
              class="w-full h-full object-contain aspect-square bg-muted/20"
            />
            <div class="absolute inset-x-0 bottom-0 bg-black/50 text-[10px] text-white px-1 py-0.5 truncate">
              {iconBuff.talentName || iconBuff.name.slice(0, 6)}
            </div>
          {:else}
            <div class="w-full h-full aspect-square flex items-center justify-center bg-muted/20 text-[11px] text-foreground p-1 text-center">
              {buff.name.slice(0, 8)}
            </div>
            <div class="absolute right-1 top-1 rounded bg-black/60 px-1 text-[9px] text-white">
              无图标
            </div>
          {/if}
        </button>
      {/each}
    </div>
  {:else}
    <div class="text-xs text-muted-foreground">
      请输入关键词搜索 Buff
    </div>
  {/if}

    <div class="space-y-2">
      <div class="text-xs text-muted-foreground">已选 Buff</div>
      <div class="flex flex-wrap gap-2">
        {#each monitoredBuffIds as buffId (buffId)}
          {@const iconBuff = selectedBuffs.find((buff) => buff.baseId === buffId)}
          {@const nameInfo = buffNames.get(buffId)}
          {#if iconBuff}
            <button
              type="button"
              class="relative rounded-md border border-border/60 overflow-hidden bg-muted/20 size-12 hover:border-border hover:bg-muted/30"
              title={iconBuff.talentName ? `${iconBuff.talentName} - ${iconBuff.name}` : iconBuff.name}
              onclick={() => toggleBuff(iconBuff.baseId)}
            >
              <img
                src={iconBuff.talentSpriteFile
                  ? `/images/talent/${iconBuff.talentSpriteFile}`
                  : `/images/buff/${iconBuff.spriteFile}`}
                alt={iconBuff.name}
                class="w-full h-full object-contain"
              />
              <div class="absolute inset-x-0 bottom-0 bg-black/60 text-[9px] text-white px-1 py-0.5 truncate">
                {iconBuff.talentName || iconBuff.name.slice(0, 6)}
              </div>
            </button>
          {:else}
            <button
              type="button"
              class="rounded-md border border-border/60 bg-muted/20 px-2 py-1 text-[11px] text-foreground hover:border-border hover:bg-muted/30"
              title={nameInfo?.name ?? `#${buffId}`}
              onclick={() => toggleBuff(buffId)}
            >
              {nameInfo?.name ?? `#${buffId}`}
            </button>
          {/if}
        {/each}
        {#if monitoredBuffIds.length === 0}
          <div class="text-xs text-muted-foreground">未选择 Buff</div>
        {/if}
      </div>
    </div>
  </div>

  <div class="rounded-lg border border-border/60 bg-card/40 p-4 space-y-3 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.02)]">
    <div>
      <h2 class="text-base font-semibold text-foreground">监控预览</h2>
      <p class="text-xs text-muted-foreground">按选择顺序排列</p>
    </div>
    <div class="grid grid-cols-5 gap-2">
      {#each Array(10) as _, idx (idx)}
        {@const skillId = monitoredSkillIds[idx]}
        {@const skill = skillId ? findAnySkillByBaseId(selectedClassKey, skillId) : undefined}
        <button
          type="button"
          class="relative rounded-md border border-border/60 overflow-hidden bg-muted/20 aspect-square text-left {skillId
            ? 'hover:border-border hover:bg-muted/30'
            : ''}"
          onclick={() => {
            if (skillId) toggleSkill(skillId);
          }}
        >
          {#if skill?.imagePath}
            <img
              src={skill.imagePath}
              alt={skill.name}
              class="w-full h-full object-cover"
            />
          {:else if skillId}
            <div class="w-full h-full flex items-center justify-center text-[10px] text-muted-foreground">
              #{skillId}
            </div>
          {:else}
            <div class="w-full h-full flex items-center justify-center text-[10px] text-muted-foreground">
              空
            </div>
          {/if}
        </button>
      {/each}
    </div>
  </div>
</div>
