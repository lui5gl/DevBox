<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import Box from "@lucide/svelte/icons/box";
  import SettingsIcon from "@lucide/svelte/icons/settings";
</script>

<section class="flex justify-between w-full max-w-2xl">
  <div class="flex gap-1 justify-center items-center">
    <Box class="h-4 w-4 stroke-lime-600 mb-1" />
    DevBox
  </div>

  <Button variant="outline">
    <SettingsIcon class="h-4 w-4" />
    Configuration
  </Button>
</section>
