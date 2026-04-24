<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import {
    Plus,
    Search,
    ArrowLeft,
    Server,
    Database,
    Zap,
    Globe,
    Tag,
    Activity,
    Plug,
    EllipsisVertical,
  } from "lucide-svelte";

  type ServiceRow = {
    name: string;
    version: string;
    status: "running" | "stopped";
    port: number;
    versionKey: string;
  };

  const services: ServiceRow[] = [
    {
      name: "Apache",
      version: "2.4.58",
      status: "running",
      port: 80,
      versionKey: "apache",
    },
    {
      name: "PHP",
      version: "8.2.17",
      status: "running",
      port: 9000,
      versionKey: "php",
    },
    {
      name: "MySQL",
      version: "8.0.32",
      status: "running",
      port: 3306,
      versionKey: "mysql",
    },
    {
      name: "Redis",
      version: "7.0.10",
      status: "stopped",
      port: 6379,
      versionKey: "redis",
    },
  ];

  type ServiceState = {
    selectedVersion: string;
    availableVersions: string[];
    isPickerOpen: boolean;
    isAddingVersion: boolean;
    search: string;
    isLoading: boolean;
    loadError: string;
  };

  let serviceStates: Record<string, ServiceState> = $state(
    Object.fromEntries(
      services.map((s) => [
        s.versionKey,
        {
          selectedVersion: s.version,
          availableVersions: [],
          isPickerOpen: false,
          isAddingVersion: false,
          search: "",
          isLoading: false,
          loadError: "",
        },
      ]),
    ),
  );

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

  const loadVersions = async (serviceKey: string) => {
    const state = serviceStates[serviceKey];
    state.isLoading = true;
    state.loadError = "";

    try {
      const versions = await invoke<string[]>(`fetch_${serviceKey}_versions`);
      const parsed = versions
        .map((item) => normalizeVersion(item))
        .filter((value): value is string => Boolean(value));

      state.availableVersions = [...new Set(parsed)].sort(compareVersionsDesc);
    } catch {
      state.loadError = `No se pudieron cargar las versiones de ${serviceKey.toUpperCase()} desde el backend.`;
    } finally {
      state.isLoading = false;
    }
  };

  const togglePicker = (serviceKey: string) => {
    const state = serviceStates[serviceKey];
    state.isPickerOpen = !state.isPickerOpen;
    if (state.isPickerOpen) {
      state.isAddingVersion = false;
      state.search = "";
    }
  };

  const selectVersion = (serviceKey: string, version: string) => {
    const state = serviceStates[serviceKey];
    state.selectedVersion = version;
    state.isPickerOpen = false;
    state.isAddingVersion = false;
    state.search = "";
  };

  const openAddMode = (serviceKey: string) => {
    const state = serviceStates[serviceKey];
    state.isAddingVersion = true;
  };

  const closeAddMode = (serviceKey: string) => {
    const state = serviceStates[serviceKey];
    state.isAddingVersion = false;
    state.search = "";
  };

  const filteredVersions = (serviceKey: string) => {
    const state = serviceStates[serviceKey];
    return state.availableVersions.filter((version) =>
      version.toLowerCase().includes(state.search.toLowerCase()),
    );
  };

  const suggestedVersions = (serviceKey: string) => {
    return serviceStates[serviceKey].availableVersions.slice(0, 12);
  };

  onMount(() => {
    services.forEach((s) => loadVersions(s.versionKey));
  });
</script>

<div class="w-full max-w-3xl mt-4 rounded-md border">
  <table class="w-full text-sm">
    <thead class="bg-muted/50 text-left">
      <tr>
        <th class="p-3">
          <div class="flex items-center gap-2">
            <Server class="h-4 w-4" />
            Service
          </div>
        </th>
        <th class="p-3">
          <div class="flex items-center gap-2">
            <Tag class="h-4 w-4" />
            Version
          </div>
        </th>
        <th class="p-3">
          <div class="flex items-center gap-2">
            <Activity class="h-4 w-4" />
            Status
          </div>
        </th>
        <th class="p-3">
          <div class="flex items-center gap-2">
            <Plug class="h-4 w-4" />
            Port
          </div>
        </th>
        <th> </th>
      </tr>
    </thead>
    <tbody>
      {#each services as service}
        {@const state = serviceStates[service.versionKey]}
        {@const ServiceIcon =
          service.versionKey === "apache" || service.versionKey === "php"
            ? Globe
            : service.versionKey === "mysql"
              ? Database
              : Zap}
        <tr class="border-t align-top">
          <td class="p-3">
            <div class="flex items-center gap-2">
              <ServiceIcon class="h-4 w-4 text-muted-foreground" />
              {service.name}
            </div>
          </td>
          <td class="p-3">{state.selectedVersion}</td>
          <td class="p-3">{service.status}</td>
          <td class="p-3">{service.port}</td>
          <td class="p-3">
            <div class="relative">
              <button
                type="button"
                class="flex h-8 w-8 items-center justify-center rounded border text-muted-foreground hover:bg-muted hover:text-foreground"
                aria-label={`Opciones de ${service.name}`}
                title={`Opciones de ${service.name}`}
                onclick={() => togglePicker(service.versionKey)}
              >
                <EllipsisVertical class="h-4 w-4" />
              </button>

              {#if state.isPickerOpen}
                <div
                  class="absolute right-0 z-10 mt-2 w-72 rounded-md border bg-background p-3 shadow-lg"
                >
                  {#if state.isAddingVersion}
                    <p class="mb-2 text-xs text-muted-foreground">
                      Buscar versiones de {service.name}
                    </p>
                  {:else}
                    <p class="mb-2 text-xs text-muted-foreground">
                      Versiones disponibles de {service.name}
                    </p>
                  {/if}

                  {#if state.isLoading}
                    <p class="text-xs text-muted-foreground">
                      Cargando versiones...
                    </p>
                  {:else if state.loadError}
                    <p class="text-xs text-red-500">{state.loadError}</p>
                  {:else if state.isAddingVersion}
                    <div class="relative">
                      <Search
                        class="absolute left-2 top-1/2 h-3 w-3 -translate-y-1/2 text-muted-foreground"
                      />
                      <input
                        type="text"
                        bind:value={state.search}
                        placeholder="Buscar versión..."
                        class="mb-2 w-full rounded border pl-8 pr-2 py-1 text-xs"
                      />
                    </div>

                    {#if filteredVersions(service.versionKey).length === 0}
                      <p class="text-xs text-muted-foreground">
                        No hay resultados para tu búsqueda.
                      </p>
                    {:else}
                      <div class="max-h-48 overflow-y-auto rounded border">
                        {#each filteredVersions(service.versionKey) as version}
                          <button
                            type="button"
                            class="block w-full border-b px-2 py-1 text-left text-xs last:border-b-0 hover:bg-muted"
                            onclick={() =>
                              selectVersion(service.versionKey, version)}
                          >
                            {version}
                          </button>
                        {/each}
                      </div>
                    {/if}

                    <button
                      type="button"
                      class="mt-2 flex w-full items-center justify-center gap-1 rounded border px-2 py-1 text-xs hover:bg-muted"
                      onclick={() => closeAddMode(service.versionKey)}
                    >
                      <ArrowLeft class="h-3 w-3" />
                      Volver
                    </button>
                  {:else}
                    {#if suggestedVersions(service.versionKey).length > 0}
                      <div class="max-h-48 overflow-y-auto rounded border">
                        {#each suggestedVersions(service.versionKey) as version}
                          <button
                            type="button"
                            class="block w-full border-b px-2 py-1 text-left text-xs last:border-b-0 hover:bg-muted"
                            onclick={() =>
                              selectVersion(service.versionKey, version)}
                          >
                            {version}
                          </button>
                        {/each}
                      </div>
                    {:else}
                      <p class="text-xs text-muted-foreground">
                        No hay versiones disponibles por ahora.
                      </p>
                    {/if}

                    <button
                      type="button"
                      class="mt-2 flex w-full items-center justify-center gap-1 rounded border px-2 py-1 text-xs hover:bg-muted"
                      onclick={() => openAddMode(service.versionKey)}
                    >
                      <Plus class="h-3 w-3" />
                      Añadir versión
                    </button>
                  {/if}
                </div>
              {/if}
            </div>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
