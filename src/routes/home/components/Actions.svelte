<script lang="ts">
  import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
  import GlobeIcon from "@lucide/svelte/icons/globe";
  import PlayIcon from "@lucide/svelte/icons/play";
  import SettingsIcon from "@lucide/svelte/icons/settings";
</script>

<section
  class="flex w-full max-w-3xl flex-wrap items-center justify-end gap-2 border-t pt-3"
>
  <button
    type="button"
    class="inline-flex h-8 items-center gap-2 rounded-md border border-transparent bg-transparent px-2 text-sm font-medium text-foreground transition-colors hover:border-input hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring"
  >
    <PlayIcon class="h-4 w-4" />
    Start
  </button>
  <button
    type="button"
    class="inline-flex h-8 items-center gap-2 rounded-md border border-transparent bg-transparent px-2 text-sm font-medium text-foreground transition-colors hover:border-input hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring"
  >
    <GlobeIcon class="h-4 w-4" />
    Web
  </button>
  <button
    type="button"
    class="inline-flex h-8 items-center gap-2 rounded-md border border-transparent bg-transparent px-2 text-sm font-medium text-muted-foreground transition-colors hover:border-input hover:bg-accent hover:text-foreground focus-visible:ring-2 focus-visible:ring-ring"
  >
    <FolderOpenIcon class="h-4 w-4" />
    Root
  </button>
  <button
    type="button"
    class="inline-flex h-8 items-center gap-2 rounded-md border border-transparent bg-transparent px-2 text-sm font-medium text-foreground transition-colors hover:border-input hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring"
  >
    <SettingsIcon class="h-4 w-4" />
    Settings
  </button>
</section>
