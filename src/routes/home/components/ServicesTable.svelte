<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    Plus,
    Search,
    ArrowLeft,
    Server,
    Database,
    CircleStop,
    Zap,
    Globe,
    Tag,
    Activity,
    Ban,
    Plug,
    ChevronDown,
    Download,
    Check,
    CheckCircle2,
    Play,
  } from "lucide-svelte";

  type ServiceRow = {
    name: string;
    version: string;
    status: ServiceStatus;
    port: number;
    versionKey: string;
  };

  type ServiceStatus = "running" | "stopped" | "disabled";

  const statusOptions: {
    value: ServiceStatus;
    label: string;
    description: string;
    icon: typeof Play;
  }[] = [
    {
      value: "running",
      label: "Running",
      description: "Service is active",
      icon: Play,
    },
    {
      value: "stopped",
      label: "Stopped",
      description: "Service is off",
      icon: CircleStop,
    },
    {
      value: "disabled",
      label: "Disabled",
      description: "Service cannot auto-start",
      icon: Ban,
    },
  ];

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
    downloadedVersions: string[];
    availableVersions: string[];
    isPickerOpen: boolean;
    isAddingVersion: boolean;
    search: string;
    isLoading: boolean;
    loadError: string;
    status: ServiceStatus;
    isStatusPickerOpen: boolean;
    port: number;
    portDraft: string;
    portError: string;
    isPortEditorOpen: boolean;
  };

  let serviceStates: Record<string, ServiceState> = $state(
    Object.fromEntries(
      services.map((s) => [
        s.versionKey,
        {
          selectedVersion: s.version,
          downloadedVersions: [s.version],
          availableVersions: [],
          isPickerOpen: false,
          isAddingVersion: false,
          search: "",
          isLoading: false,
          loadError: "",
          status: s.status,
          isStatusPickerOpen: false,
          port: s.port,
          portDraft: String(s.port),
          portError: "",
          isPortEditorOpen: false,
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
      state.loadError = `Could not load ${serviceKey.toUpperCase()} versions from the backend.`;
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
    if (!state.downloadedVersions.includes(version)) {
      state.downloadedVersions = [...state.downloadedVersions, version].sort(
        compareVersionsDesc,
      );
    }
    state.isPickerOpen = false;
    state.isAddingVersion = false;
    state.search = "";
  };

  const openAddMode = (serviceKey: string) => {
    const state = serviceStates[serviceKey];
    state.isAddingVersion = true;
    state.search = "";

    if (state.availableVersions.length === 0 && !state.isLoading) {
      void loadVersions(serviceKey);
    }
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

  const downloadedVersions = (serviceKey: string) => {
    return serviceStates[serviceKey].downloadedVersions;
  };

  const isDownloadedVersion = (serviceKey: string, version: string) => {
    return serviceStates[serviceKey].downloadedVersions.includes(version);
  };

  const statusLabel = (status: ServiceStatus) => {
    return statusOptions.find((option) => option.value === status)?.label ?? status;
  };

  const statusClass = (status: ServiceStatus) => {
    if (status === "disabled") return "text-muted-foreground";
    return "";
  };

  const toggleStatusPicker = (serviceKey: string) => {
    const state = serviceStates[serviceKey];
    state.isStatusPickerOpen = !state.isStatusPickerOpen;
  };

  const selectStatus = (serviceKey: string, status: ServiceStatus) => {
    const state = serviceStates[serviceKey];
    state.status = status;
    state.isStatusPickerOpen = false;
  };

  const validatePort = (serviceKey: string) => {
    const state = serviceStates[serviceKey];
    const port = Number(state.portDraft);

    if (!Number.isInteger(port) || port < 1 || port > 65535) {
      state.portError = "Use a port from 1 to 65535.";
      return;
    }

    state.port = port;
    state.portDraft = String(port);
    state.portError = "";
    state.isPortEditorOpen = false;
  };

  const togglePortEditor = (serviceKey: string) => {
    const state = serviceStates[serviceKey];
    state.isPortEditorOpen = !state.isPortEditorOpen;
    state.portDraft = String(state.port);
    state.portError = "";
  };
</script>

<div class="w-full max-w-3xl rounded-md border">
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
        <tr class="border-t align-middle">
          <td class="p-3">
            <div class="flex items-center gap-2">
              <ServiceIcon class="h-4 w-4 text-muted-foreground" />
              {service.name}
            </div>
          </td>
          <td class="p-3">
            <div class="relative inline-block">
              <button
                type="button"
                class="inline-flex h-8 items-center gap-2 rounded border px-2 text-left hover:bg-muted"
                aria-label={`Select ${service.name} version`}
                title={`Select ${service.name} version`}
                onclick={() => togglePicker(service.versionKey)}
              >
                <span>{state.selectedVersion}</span>
                <ChevronDown class="h-3 w-3 text-muted-foreground" />
              </button>

              {#if state.isPickerOpen}
                <div
                  class="absolute left-0 z-10 mt-2 w-72 rounded-md border bg-background p-3 shadow-lg"
                >
                  {#if state.isAddingVersion}
                    <p class="mb-2 text-xs text-muted-foreground">
                      Search {service.name} versions
                    </p>
                  {:else}
                    <p class="mb-2 text-xs text-muted-foreground">
                      Installed {service.name} versions
                    </p>
                  {/if}

                  {#if state.isAddingVersion}
                    {#if state.isLoading}
                      <p class="text-xs text-muted-foreground">
                        Loading versions...
                      </p>
                    {:else if state.loadError}
                      <p class="text-xs text-red-500">{state.loadError}</p>
                    {:else}
                      <div class="relative mb-2">
                        <Search
                          class="absolute left-2 top-1/2 h-3 w-3 -translate-y-1/2 text-muted-foreground"
                        />
                        <input
                          type="text"
                          bind:value={state.search}
                          placeholder="Search version..."
                          class="w-full rounded border pl-8 pr-2 py-1 text-xs"
                        />
                      </div>

                      {#if filteredVersions(service.versionKey).length === 0}
                        <p class="text-xs text-muted-foreground">
                          No results for your search.
                        </p>
                      {:else}
                        <div class="max-h-48 overflow-y-auto rounded border">
                          {#each filteredVersions(service.versionKey) as version}
                            <button
                              type="button"
                              class="flex w-full items-center justify-between gap-2 border-b px-2 py-1 text-left text-xs last:border-b-0 hover:bg-muted"
                              onclick={() =>
                                selectVersion(service.versionKey, version)}
                            >
                              <span>{version}</span>
                              {#if isDownloadedVersion(service.versionKey, version)}
                                <CheckCircle2
                                  class="h-3 w-3 shrink-0 text-emerald-500"
                                />
                              {:else}
                                <Download
                                  class="h-3 w-3 shrink-0 text-muted-foreground"
                                />
                              {/if}
                            </button>
                          {/each}
                        </div>
                      {/if}
                    {/if}

                    <button
                      type="button"
                      class="mt-2 flex w-full items-center justify-center gap-1 rounded border px-2 py-1 text-xs hover:bg-muted"
                      onclick={() => closeAddMode(service.versionKey)}
                    >
                      <ArrowLeft class="h-3 w-3" />
                      Back
                    </button>
                  {:else}
                    {#if downloadedVersions(service.versionKey).length > 0}
                      <div class="max-h-48 overflow-y-auto rounded border">
                        {#each downloadedVersions(service.versionKey) as version}
                          <button
                            type="button"
                            class="flex w-full items-center justify-between gap-2 border-b px-2 py-1 text-left text-xs last:border-b-0 hover:bg-muted"
                            onclick={() =>
                              selectVersion(service.versionKey, version)}
                          >
                            <span>{version}</span>
                            <CheckCircle2
                              class="h-3 w-3 shrink-0 text-emerald-500"
                            />
                          </button>
                        {/each}
                      </div>
                    {:else}
                      <p class="text-xs text-muted-foreground">
                        No installed versions yet.
                      </p>
                    {/if}

                    <button
                      type="button"
                      class="mt-2 flex w-full items-center justify-center gap-1 rounded border px-2 py-1 text-xs hover:bg-muted"
                      onclick={() => openAddMode(service.versionKey)}
                    >
                      <Plus class="h-3 w-3" />
                      Search another version
                    </button>
                  {/if}
                </div>
              {/if}
            </div>
          </td>
          <td class="p-3">
            <div class="relative inline-block">
              <button
                type="button"
                class={`inline-flex h-8 items-center gap-2 rounded border px-2 text-left hover:bg-muted ${statusClass(state.status)}`}
                aria-label={`Change ${service.name} status`}
                title={`Change ${service.name} status`}
                onclick={() => toggleStatusPicker(service.versionKey)}
              >
                <span>{statusLabel(state.status)}</span>
                <ChevronDown class="h-3 w-3" />
              </button>

              {#if state.isStatusPickerOpen}
                <div
                  class="absolute left-0 z-10 mt-2 w-56 rounded-md border bg-background p-2 shadow-lg"
                >
                  {#each statusOptions as option}
                    {@const StatusIcon = option.icon}
                    <button
                      type="button"
                      class="flex w-full items-center justify-between gap-2 rounded px-2 py-2 text-left text-xs hover:bg-muted"
                      onclick={() =>
                        selectStatus(service.versionKey, option.value)}
                    >
                      <span class="flex min-w-0 items-center gap-2">
                        <StatusIcon
                          class="h-4 w-4 shrink-0 text-muted-foreground"
                        />
                        <span class="min-w-0">
                          <span class="block font-medium">{option.label}</span>
                          <span class="block text-muted-foreground">
                            {option.description}
                          </span>
                        </span>
                      </span>
                      {#if state.status === option.value}
                        <Check class="h-3 w-3 shrink-0 text-muted-foreground" />
                      {/if}
                    </button>
                  {/each}
                </div>
              {/if}
            </div>
          </td>
          <td class="p-3">
            <div class="relative inline-block">
              <button
                type="button"
                class="inline-flex h-8 items-center gap-2 rounded border px-2 text-left hover:bg-muted"
                aria-label={`Change ${service.name} port`}
                title={`Change ${service.name} port`}
                onclick={() => togglePortEditor(service.versionKey)}
              >
                <span>{state.port}</span>
                <ChevronDown class="h-3 w-3 text-muted-foreground" />
              </button>

              {#if state.isPortEditorOpen}
                <div
                  class="absolute right-0 z-10 mt-2 w-56 rounded-md border bg-background p-3 shadow-lg"
                >
                  <p class="mb-2 text-xs text-muted-foreground">
                    Change {service.name} port
                  </p>
                  <div class="flex items-center gap-2">
                    <input
                      type="text"
                      inputmode="numeric"
                      bind:value={state.portDraft}
                      aria-label={`${service.name} port`}
                      placeholder="Port"
                      class="h-8 min-w-0 flex-1 rounded border px-2 text-sm"
                    />
                    <button
                      type="button"
                      class="inline-flex h-8 items-center justify-center gap-1 rounded border px-2 text-xs hover:bg-muted"
                      aria-label={`Validate ${service.name} port`}
                      title="Validate"
                      onclick={() => validatePort(service.versionKey)}
                    >
                      <Check class="h-3 w-3" />
                      Validate
                    </button>
                  </div>
                  {#if state.portError}
                    <p class="mt-2 text-xs text-red-500">{state.portError}</p>
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
