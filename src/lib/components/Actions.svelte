<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import GlobeIcon from "@lucide/svelte/icons/globe";
  import PlayIcon from "@lucide/svelte/icons/play";
  import ShieldIcon from "@lucide/svelte/icons/shield";
  import TerminalIcon from "@lucide/svelte/icons/terminal";
</script>

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
