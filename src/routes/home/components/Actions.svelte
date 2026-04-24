<script lang="ts">
  import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
  import GlobeIcon from "@lucide/svelte/icons/globe";
  import PlayIcon from "@lucide/svelte/icons/play";
  import SettingsIcon from "@lucide/svelte/icons/settings";
</script>

<section class="mx-auto flex flex-wrap items-center justify-center gap-2">
  <button
    type="button"
    class="inline-flex items-center gap-2 rounded-md bg-primary px-3 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90 focus-visible:ring-2 focus-visible:ring-ring"
  >
    <PlayIcon class="h-4 w-4" />
    Start
  </button>
  <button
    type="button"
    class="inline-flex items-center gap-2 rounded-md border border-input bg-background px-3 py-2 text-sm font-medium text-foreground transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:ring-2 focus-visible:ring-ring"
  >
    <GlobeIcon class="h-4 w-4" />
    Web
  </button>
  <button
    type="button"
    class="inline-flex items-center gap-2 rounded-md border border-input bg-background px-3 py-2 text-sm font-medium text-foreground transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:ring-2 focus-visible:ring-ring"
  >
    <SettingsIcon class="h-4 w-4" />
    Settings
  </button>
  <button
    type="button"
    class="inline-flex items-center gap-2 rounded-md border border-input bg-background px-3 py-2 text-sm font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:ring-2 focus-visible:ring-ring"
  >
    <FolderOpenIcon class="h-4 w-4" />
    Root
  </button>
</section>
