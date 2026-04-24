import type { ColumnDef } from "@tanstack/svelte-table";

export type Service = {
  id: string;
  name: string;
  version: string;
  status: "running" | "stopped";
  port: number;
};

export const columns: ColumnDef<Service>[] = [
  {
    accessorKey: "name",
    header: "Service",
  },
  {
    accessorKey: "version",
    header: "Version",
  },
  {
    accessorKey: "port",
    header: "Port",
  },
  {
    accessorKey: "status",
    header: "Status",
  },
];