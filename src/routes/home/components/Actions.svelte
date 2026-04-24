<script lang="ts">
  import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
  import GlobeIcon from "@lucide/svelte/icons/globe";
  import PlayIcon from "@lucide/svelte/icons/play";
  import SettingsIcon from "@lucide/svelte/icons/settings";
</script>

<section class="mx-auto flex flex-wrap items-center justify-center gap-2">
  <button
    type="button"
    class="inline-flex items-center gap-2 rounded-md border px-3 py-2 text-sm hover:bg-muted"
  >
    <PlayIcon class="h-4 w-4" />
    Start
  </button>
  <button
    type="button"
    class="inline-flex items-center gap-2 rounded-md border px-3 py-2 text-sm hover:bg-muted"
  >
    <GlobeIcon class="h-4 w-4" />
    Web
  </button>
  <button
    type="button"
    class="inline-flex items-center gap-2 rounded-md border px-3 py-2 text-sm hover:bg-muted"
  >
    <SettingsIcon class="h-4 w-4" />
    Settings
  </button>
  <button
    type="button"
    class="inline-flex items-center gap-2 rounded-md border px-3 py-2 text-sm hover:bg-muted"
  >
    <FolderOpenIcon class="h-4 w-4" />
    Root
  </button>
</section>
