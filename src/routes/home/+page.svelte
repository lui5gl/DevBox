<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import Box from "@lucide/svelte/icons/box";
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import GlobeIcon from "@lucide/svelte/icons/globe";
  import PlayIcon from "@lucide/svelte/icons/play";
  import SettingsIcon from "@lucide/svelte/icons/settings";
  import ShieldIcon from "@lucide/svelte/icons/shield";
  import TerminalIcon from "@lucide/svelte/icons/terminal";
</script>

<div class="flex flex-col items-center gap-2">
  <section class="flex justify-between w-full max-w-2xl">
    <div class="flex gap-1 justify-center items-center">
      <Box class="h-4 w-4 stroke-lime-600 mb-1" />
      DevHome
    </div>

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
