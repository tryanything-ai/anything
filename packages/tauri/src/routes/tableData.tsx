import { createColumnHelper ,
  flexRender,
  getCoreRowModel,
  getSortedRowModel,
  SortingState,
  useReactTable,
} from "@tanstack/react-table";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

import { useSqlContext } from "../context/SqlProvider";

interface Event {
  event_id: string;
  session_id: string;
  node_id: string;
  node_type: string;
  node_label: string; 
  flow_id: string;
  flow_name: string;
  flow_version: string;
  worker_type: string;
  worker_name: string;
  stage: string;
  event_status: string;
  session_status: string;
  created_at: Date;
  event_result: string;
  event_context: string;
  data: string;
}

const columnHelper = createColumnHelper<Event>();

export const eventColumnDefs = [
  columnHelper.accessor((event: Event) => event.event_id, {
    id: "event_id",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Event ID</span>,
  }),
  columnHelper.accessor((event: Event) => event.session_id, {
    id: "session_id",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Session ID</span>,
  }),
  columnHelper.accessor((event: Event) => event.node_id, {
    id: "node_id",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Node ID</span>,
  }),
  columnHelper.accessor((event: Event) => event.node_type, {
    id: "node_type",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Node Type</span>,
  }),
  columnHelper.accessor((event: Event) => event.node_label, {
    id: "node_label",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Node Label</span>,
  }),
  columnHelper.accessor((event: Event) => event.flow_id, {
    id: "flow_id",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Flow ID</span>,
  }),
  columnHelper.accessor((event: Event) => event.flow_name, {
    id: "flow_name",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Flow Name</span>,
  }),
  columnHelper.accessor((event: Event) => event.flow_version, {
    id: "flow_version",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Flow Version</span>,
  }),
  columnHelper.accessor((event: Event) => event.worker_type, {
    id: "worker_type",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Worker Type</span>,
  }),
  columnHelper.accessor((event: Event) => event.worker_name, {
    id: "worker_name",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Worker Name</span>,
  }),
  columnHelper.accessor((event: Event) => event.stage, {
    id: "stage",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Stage</span>,
  }),
  columnHelper.accessor((event: Event) => event.event_status, {
    id: "event_status",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Event Status</span>,
  }),
  columnHelper.accessor((event: Event) => event.session_status, {
    id: "session_status",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Session Status</span>,
  }),
  columnHelper.accessor((event: Event) => event.created_at, {
    id: "created_at",
    cell: (info) => <span>{new Date(info.getValue()).toLocaleString()}</span>,
    header: () => <span>Created At</span>,
  }),
  columnHelper.accessor((event: Event) => event.event_context, {
    id: "event_context",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Event Context</span>,
  }),
  columnHelper.accessor((event: Event) => event.event_result, {
    id: "event_result",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Event Result</span>,
  }),
  columnHelper.accessor((event: Event) => event.data, {
    id: "data",
    cell: (info) => <span>{info.getValue()}</span>,
    header: () => <span>Data</span>,  
  }),
];

export default function Tables() {
  const { getTableData } = useSqlContext();
  const [data, setData] = useState<any[]>([]);
  const { table } = useParams();

  const [sorting, setSorting] = useState<SortingState>([]);

  const react_table = useReactTable({
    columns: eventColumnDefs,
    data: data as Event[],
    getCoreRowModel: getCoreRowModel(),
    //2. add getSortedRowModel into the pipeline. this will calculate the sorted rows when the sort state changes
    getSortedRowModel: getSortedRowModel(),
    //3. add state to our table we use the  sorting state from step 1
    state: {
      sorting,
    },
    //4. add a handler for onSortingChange using the setSorting from step 1
    onSortingChange: setSorting,
    // columns: eventColumnDefs,
    // data: data ?? [],
    // getCoreRowModel: getCoreRowModel(),
  });

  const headers = react_table.getFlatHeaders();
  const rows = react_table.getRowModel().rows;

  const hydrate = async () => {
    try {
      if (!table) return;
      const data = await getTableData(table);
      setData(data);
    } catch (error) {
      console.log("error", error);
    }
  };

  useEffect(() => {
    hydrate();

    // Set up the interval to call hydrate every one second
    const intervalId = setInterval(hydrate, 1000);

    // Clean up the interval when the component unmounts
    return () => clearInterval(intervalId);
  }, []);
  return (
    <div className="flex flex-col h-full w-full">
      <div className="flex flex-row">
        <div className="text-5xl text-white m-5">table/{table}</div>
        <button onClick={hydrate}>Refresh</button>
      </div>
      <div className="overflow-y-auto overflow-x-auto">
        {/* <table className="table table-xs"> */}
        <table className="table table-zebra my-4 w-full">
          <thead>
            <tr>
              {headers.map((header) => {
                //5. check if the column is sorted
                const direction = header.column.getIsSorted();

                //6. create a map to get the sorting indicator
                const arrow: any = {
                  asc: "🔼",
                  desc: "🔽",
                };

                //6. get the sorting indicator if header is sorted
                const sort_indicator = direction && arrow[direction];
                return (
                  <th key={header.id}>
                    {header.isPlaceholder ? null : (
                      //7. add an onClick handler using header.column.getToggleSortingHandler
                      <div
                        onClick={header.column.getToggleSortingHandler()}
                        // 8. add a class to render the sorting indicator properly
                        className="cursor-pointer flex gap-4"
                      >
                        {flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                        {/* 9. render the sorting indicator */}
                        {direction && <span>{sort_indicator}</span>}
                      </div>
                    )}
                  </th>
                );
              })}
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr key={row.id}>
                {row.getVisibleCells().map((cell) => (
                  <td key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
