<script lang="ts">
  import { createSvelteTable, getCoreRowModel, flexRender } from "@tanstack/svelte-table";
  import * as Table from "$lib/components/ui/table/index.js";

  let { columns, data } = $props();

  const table = createSvelteTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });
</script>

<div class="rounded-md border">
  <Table.Root>
    <Table.Header>
      {#each $table.getHeaderGroups() as headerGroup}
        <Table.Row>
          {#each headerGroup.headers as header}
            <Table.Head>
              {@render flexRender(header.column.columnDef.header, header.getContext())}
            </Table.Head>
          {/each}
        </Table.Row>
      {#/each}
    </Table.Header>
    <Table.Body>
      {#each $table.getRowModel().rows as row}
        <Table.Row>
          {#each row.getVisibleCells() as cell}
            <Table.Cell>
              {@render flexRender(cell.column.columnDef.cell, cell.getContext())}
            </Table.Cell>
          {#/each}
        </Table.Row>
      {:else}
        <Table.Row>
          <Table.Cell colspan={columns.length} class="h-24 text-center">No results.</Table.Cell>
        </Table.Row>
      {#/each}
    </Table.Body>
  </Table.Root>
</div>