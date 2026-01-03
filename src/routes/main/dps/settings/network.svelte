<script lang="ts">
    import SettingsSelect from "./settings-select.svelte";
    import SettingsDropdown from "./settings-dropdown.svelte";
    import { SETTINGS } from "$lib/settings-store";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { untrack } from "svelte";

    type Device = {
        name: string;
        description: string | null;
    };

    let devices = $state<Device[]>([]);
    let npcapInstalled = $state(false);
    let loading = $state(false);
    let mounted = $state(false);
    // Track initial values to detect actual user changes
    let initialMethod = $state<string | null>(null);
    let initialDevice = $state<string | null>(null);

    async function loadDevices() {
        loading = true;
        try {
            npcapInstalled = await invoke("check_npcap_status");
            if (npcapInstalled) {
                devices = await invoke("get_network_devices");
            }
        } catch (e) {
            console.error("Failed to load network info", e);
        }
        loading = false;
    }

    onMount(() => {
        // Capture initial values before marking as mounted
        // Use untrack to avoid reactive dependencies
        untrack(() => {
            initialMethod = SETTINGS.packetCapture.state.method;
            initialDevice = SETTINGS.packetCapture.state.npcapDevice;
        });
        mounted = true;
        loadDevices();
    });

    $effect(() => {
        if (!mounted) return;
        const method = SETTINGS.packetCapture.state.method;
        const device = SETTINGS.packetCapture.state.npcapDevice;

        // Skip saving if values haven't changed from initial (prevents overwriting on mount)
        if (initialMethod !== null && method === initialMethod && device === initialDevice) {
            return;
        }

        // Update tracked values for future comparisons
        initialMethod = method;
        initialDevice = device;

        invoke("save_packet_capture_settings", {
            method,
            npcapDevice: device,
        }).catch((e) => console.error("Failed to save packet capture settings", e));
    });

    let deviceOptions = $derived(
        devices.map((d) => ({
            value: d.name,
            label: d.description || d.name,
        })),
    );
</script>

<div class="space-y-3">
    <div
        class="rounded-lg border bg-card/40 border-border/60 overflow-hidden shadow-[inset_0_1px_0_0_rgba(255,255,255,0.02)]"
    >
        <div class="px-4 py-3">
            <h2 class="text-base font-semibold text-foreground mb-2">
                抓包
            </h2>

            <SettingsSelect
                bind:selected={SETTINGS.packetCapture.state.method}
                label="捕获方式"
                description="选择用于捕获网络数据包的方法（需要重启应用）。"
                values={["WinDivert", "Npcap"]}
            />

            {#if SETTINGS.packetCapture.state.method === "Npcap"}
                {#if !npcapInstalled}
                    <div
                        class="mt-2 p-3 bg-destructive/10 text-destructive rounded-md text-sm"
                    >
                        未检测到 Npcap。请从 <a
                            href="https://npcap.com/"
                            target="_blank"
                            class="underline">npcap.com</a
                        > 安装 Npcap 以使用该功能。
                    </div>
                {:else}
                    <SettingsDropdown
                        bind:selected={SETTINGS.packetCapture.state.npcapDevice}
                        label="网络设备"
                        description="选择用于捕获流量的网卡。"
                        options={deviceOptions}
                        placeholder={loading
                            ? "正在加载设备..."
                            : "选择设备"}
                    />
                {/if}
            {/if}
        </div>
    </div>

</div>
