<script lang="ts">
  import ServiceActionsMenu from "./ServiceActionsMenu.svelte";

  const services = [
    ["PHP", "7.4.33", "running", 9000],
    ["Nginx", "1.24.0", "running", 80],
    ["MySQL", "8.0.32", "running", 3306],
    ["Redis", "7.0.10", "stopped", 6379],
  ];
</script>

<div class="w-full max-w-2xl mt-4 overflow-hidden rounded-md border">
  <table class="w-full text-sm">
    <thead class="bg-muted/50 text-left"><tr><th class="p-3">Service</th><th class="p-3">Version</th><th class="p-3">Status</th><th class="p-3">Port</th></tr></thead>
    <tbody>{#each services as [name, version, status, port]}<tr class="border-t"><td class="p-3">{name}</td><td class="p-3">{version}</td><td class="p-3">{status}</td><td class="p-3">{port}</td><td class="p-3"><ServiceActionsMenu /></td></tr>{/each}</tbody>
  </table>
</div>