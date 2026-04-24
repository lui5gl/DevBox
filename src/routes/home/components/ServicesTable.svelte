<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  type ServiceRow = {
    name: string;
    version: string;
    status: "running" | "stopped";
    port: number;
  };

  const services: ServiceRow[] = [
    { name: "Apache", version: "2.4.58", status: "running", port: 80 },
    { name: "PHP", version: "8.2.17", status: "running", port: 9000 },
    { name: "MySQL", version: "8.0.32", status: "running", port: 3306 },
    { name: "Redis", version: "7.0.10", status: "stopped", port: 6379 },
  ];

  let apachePhpVersion = $state("8.2");
  let availablePhpVersions = $state<string[]>([]);
  let search = $state("");
  let isPickerOpen = $state(false);
  let isAddingVersion = $state(false);
  let isLoading = $state(false);
  let loadError = $state("");

  const normalizeVersion = (tag: string) => {
    const match = tag.match(/^\d+(?:\.\d+){1,2}$/);
    return match ? match[0] : null;
  };

  const compareVersionsDesc = (a: string, b: string) => {
    const pa = a.split(".").map((x) => Number(x));
    const pb = b.split(".").map((x) => Number(x));
    const size = Math.max(pa.length, pb.length);

    for (let i = 0; i < size; i += 1) {
      const av = pa[i] ?? 0;
      const bv = pb[i] ?? 0;
      if (av !== bv) return bv - av;
    }

    return 0;
  };

  const loadPhpVersions = async () => {
    isLoading = true;
    loadError = "";

    try {
      const versions = await invoke<string[]>("fetch_php_versions");
      const parsed = versions
        .map((item) => normalizeVersion(item))
        .filter((value): value is string => Boolean(value));

      availablePhpVersions = [...new Set(parsed)].sort(compareVersionsDesc);
    } catch {
      loadError = "No se pudieron cargar las versiones de PHP desde el backend.";
    } finally {
      isLoading = false;
    }
  };

  const togglePicker = () => {
    isPickerOpen = !isPickerOpen;
    if (isPickerOpen) {
      isAddingVersion = false;
      search = "";
    }
  };

  const selectVersion = (version: string) => {
    apachePhpVersion = version;
    isPickerOpen = false;
    isAddingVersion = false;
    search = "";
  };

  const openAddMode = () => {
    isAddingVersion = true;
  };

  const closeAddMode = () => {
    isAddingVersion = false;
    search = "";
  };

  const filteredVersions = $derived(
    availablePhpVersions.filter((version) => version.toLowerCase().includes(search.toLowerCase()))
  );

  const downloadedVersions = $derived(availablePhpVersions.slice(0, 12));

  onMount(() => {
    loadPhpVersions();
  });
</script>

<div class="w-full max-w-3xl mt-4 overflow-hidden rounded-md border">
  <table class="w-full text-sm">
    <thead class="bg-muted/50 text-left">
      <tr>
        <th class="p-3">Service</th>
        <th class="p-3">Version</th>
        <th class="p-3">Status</th>
        <th class="p-3">Port</th>
        <th class="p-3">Action</th>
      </tr>
    </thead>
    <tbody>
      {#each services as service}
        <tr class="border-t align-top">
          <td class="p-3">{service.name}</td>
          <td class="p-3">{service.version}</td>
          <td class="p-3">{service.status}</td>
          <td class="p-3">{service.port}</td>
          <td class="p-3">
            {#if service.name === "Apache"}
              <div class="relative">
                <button
                  type="button"
                  class="rounded border px-3 py-1 text-xs hover:bg-muted"
                  onclick={togglePicker}
                >
                  PHP {apachePhpVersion}
                </button>

                {#if isPickerOpen}
                  <div class="absolute right-0 z-10 mt-2 w-72 rounded-md border bg-background p-3 shadow-lg">
                    <p class="mb-2 text-xs text-muted-foreground">Selecciona la version de PHP para Apache</p>

                    {#if isLoading}
                      <p class="text-xs text-muted-foreground">Cargando versiones...</p>
                    {:else if loadError}
                      <p class="text-xs text-red-500">{loadError}</p>
                    {:else}
                      {#if !isAddingVersion}
                        <p class="mb-2 text-xs font-medium">Versiones descargadas</p>

                        {#if downloadedVersions.length === 0}
                          <p class="text-xs text-muted-foreground">No hay versiones disponibles.</p>
                        {:else}
                          <div class="max-h-48 overflow-y-auto rounded border">
                            {#each downloadedVersions as version}
                              <button
                                type="button"
                                class="block w-full border-b px-2 py-1 text-left text-xs last:border-b-0 hover:bg-muted"
                                onclick={() => selectVersion(version)}
                              >
                                {version}
                              </button>
                            {/each}
                          </div>
                        {/if}

                        <button
                          type="button"
                          class="mt-2 w-full rounded border px-2 py-1 text-xs hover:bg-muted"
                          onclick={openAddMode}
                        >
                          Anadir
                        </button>
                      {:else}
                        <input
                          type="text"
                          bind:value={search}
                          placeholder="Buscar version..."
                          class="mb-2 w-full rounded border px-2 py-1 text-xs"
                        />

                        {#if filteredVersions.length === 0}
                          <p class="text-xs text-muted-foreground">No hay resultados para tu busqueda.</p>
                        {:else}
                          <div class="max-h-48 overflow-y-auto rounded border">
                            {#each filteredVersions as version}
                              <button
                                type="button"
                                class="block w-full border-b px-2 py-1 text-left text-xs last:border-b-0 hover:bg-muted"
                                onclick={() => selectVersion(version)}
                              >
                                {version}
                              </button>
                            {/each}
                          </div>
                        {/if}

                        <button
                          type="button"
                          class="mt-2 w-full rounded border px-2 py-1 text-xs hover:bg-muted"
                          onclick={closeAddMode}
                        >
                          Volver
                        </button>
                      {/if}
                    {/if}
                  </div>
                {/if}
              </div>
            {:else}
              <span class="text-xs text-muted-foreground">-</span>
            {/if}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>