<script lang="ts">
  import Header from "./components/Header.svelte";
  import Actions from "./components/Actions.svelte";
  import ServicesTable from "./components/ServicesTable.svelte";
</script>

<div class="flex flex-col items-center gap-2 justify-between h-full">
  <div class="flex flex-col items-center gap-4 w-full">
    <Header />
    <ServicesTable />
  </div>
  <Actions />
</div>
