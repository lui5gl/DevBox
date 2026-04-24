<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import PlayIcon from "@lucide/svelte/icons/play";
  import GlobeIcon from "@lucide/svelte/icons/globe";
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import TerminalIcon from "@lucide/svelte/icons/terminal";
  import ShieldIcon from "@lucide/svelte/icons/shield";
  import SettingsIcon from "@lucide/svelte/icons/settings";
</script>

<div class="flex flex-col items-center gap-2">
  <section class="">
    <Button variant="outline">
      <SettingsIcon class="h-4 w-4" />
      Configuration
    </Button>
  </section>
  <section class="mx-auto">
    <Button variant="outline">
      <PlayIcon class="h-4 w-4" />
      Start
    </Button>
    <Button variant="outline">
      <GlobeIcon class="h-4 w-4" />
      Open Web
    </Button>
    <Button variant="outline">
      <DatabaseIcon class="h-4 w-4" />
      Database
    </Button>
    <Button variant="outline">
      <TerminalIcon class="h-4 w-4" />
      Terminal
    </Button>
    <Button variant="outline">
      <ShieldIcon class="h-4 w-4" />
      Root
    </Button>
  </section>
</div>
