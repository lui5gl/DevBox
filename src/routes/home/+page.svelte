<script lang="ts">
  import Actions from "./components/Actions.svelte";
  import ServicesTable from "./components/ServicesTable.svelte";
</script>

<div class="flex flex-col items-center gap-2 h-full">
  <div class="flex flex-col items-center gap-4 w-full">
    <ServicesTable />
  </div>
  <Actions />
</div>
