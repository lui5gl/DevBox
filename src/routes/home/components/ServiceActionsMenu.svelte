<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import { MoreVertical } from 'lucide-svelte';
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger class="inline-flex h-8 w-8 items-center justify-center rounded-md hover:bg-muted">
    <MoreVertical class="h-4 w-4" />
  </DropdownMenu.Trigger>
  <DropdownMenu.Content>
    <DropdownMenu.Item>Enable</DropdownMenu.Item>
    <DropdownMenu.Item>Disable</DropdownMenu.Item>
    <DropdownMenu.Item>Stop</DropdownMenu.Item>
    <DropdownMenu.Item>Restart</DropdownMenu.Item>
    <DropdownMenu.Separator />
    <DropdownMenu.Item>Delete</DropdownMenu.Item>
  </DropdownMenu.Content>
</DropdownMenu.Root>