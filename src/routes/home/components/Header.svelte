<script lang="ts">
  import Box from "@lucide/svelte/icons/box";
  import SettingsIcon from "@lucide/svelte/icons/settings";
</script>

<section class="flex justify-between w-full max-w-2xl">
  <div class="flex gap-1 justify-center items-center">
    <Box class="h-4 w-4 stroke-lime-400 mb-1" />
    DevBox
  </div>

  <button
    type="button"
    class="inline-flex items-center gap-2 rounded-md border px-3 py-2 text-sm hover:bg-muted"
  >
    <SettingsIcon class="h-4 w-4" />
    Configuration
  </button>
</section>
