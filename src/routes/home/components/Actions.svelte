<script lang="ts">
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import GlobeIcon from "@lucide/svelte/icons/globe";
  import PlayIcon from "@lucide/svelte/icons/play";
  import ShieldIcon from "@lucide/svelte/icons/shield";
  import TerminalIcon from "@lucide/svelte/icons/terminal";
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
    Open Web
  </button>
  <button
    type="button"
    class="inline-flex items-center gap-2 rounded-md border px-3 py-2 text-sm hover:bg-muted"
  >
    <DatabaseIcon class="h-4 w-4" />
    Database
  </button>
  <button
    type="button"
    class="inline-flex items-center gap-2 rounded-md border px-3 py-2 text-sm hover:bg-muted"
  >
    <TerminalIcon class="h-4 w-4" />
    Terminal
  </button>
  <button
    type="button"
    class="inline-flex items-center gap-2 rounded-md border px-3 py-2 text-sm hover:bg-muted"
  >
    <ShieldIcon class="h-4 w-4" />
    Root
  </button>
</section>
