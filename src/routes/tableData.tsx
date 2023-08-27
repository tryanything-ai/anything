import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { useSqlContext } from "../context/SqlProvider";

export default function Tables() {
  const { getTableData } = useSqlContext();
  const [data, setData] = useState<any[]>([]);
  const { table } = useParams();

  const hydrate = async () => {
    try {
      if (!table) return;
      const data = await getTableData(table);
      console.log("data in tableData", data);
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
    <div className="flex flex-col h-full w-full m-10">
      <div className="flex flex-row">
        <div className="text-5xl text-white m-5">table/{table}</div>
        <button onClick={hydrate}>Refresh</button>
      </div>
      <div className="overflow-y-auto max-h-[600px]">
        <table className="table table-xs">
          <thead>
            <tr>
              <th>Event ID</th>
              <th>Created At</th>
              <th>Node Id</th>
              <th>Node Type</th>
              <th>Flow ID</th>
              <th>Flow Name</th>
              <th>Flow Version</th>
              <th>Worker Type</th>
              <th>Stage</th>
              <th>Event Status</th>
              <th>Session Status</th>
              <th>Data</th>
            </tr>
          </thead>
          <tbody>
            {data.map((event: any) => {
              return (
                <tr>
                  <th>{event.event_id}</th>
                  <th>{event.created_at}</th>
                  <th>{event.node_id}</th>
                  <th>{event.node_type}</th>
                  <th>{event.flow_id}</th>
                  <th>{event.flow_name}</th>
                  <th>{event.flow_version}</th>
                  <th>{event.worker_type}</th>
                  <th>{event.stage}</th>
                  <th>{event.event_status}</th>
                  <th>{event.session_status}</th>
                  <th>{JSON.stringify(event.data)}</th>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}
