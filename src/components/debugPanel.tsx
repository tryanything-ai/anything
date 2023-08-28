import { useState, useEffect } from "react";
import { useSqlContext } from "../context/SqlProvider";
import { useParams } from "react-router-dom";

const DebugPanel = () => {
  const { getFlowEvents } = useSqlContext();
  const { flow_name } = useParams();
  const [events, setEvents] = useState<any[]>([]);

  const hydrate = async () => {
    try {
      if (!flow_name) return;
      const data = await getFlowEvents(flow_name);
      setEvents(data);
    } catch (error) {
      console.log("error", error);
    }
  };

  useEffect(() => {
    hydrate();
    const intervalId = setInterval(hydrate, 500);
    return () => clearInterval(intervalId);
  }, []);

  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500">
      {events.length > 0 ? (
        <>
          <div className="text-2xl font-bold">Processing Tasks</div>
          <ul>
            {events.map((event) => {
              return (
                <div
                  key={event.event_id}
                  className="card h-20 w-full text-md text-primary-content border p-4 my-2"
                >
                  {event.node_type}
                </div>
              );
            })}
          </ul>
        </>
      ) : (
        <div className="flex-1 text-center">
          <div>
            <h1 className="text-2xl font-bold">No Tasks</h1>
            <p className="text-sm p-2">
              Tasks will appear here when your flow runs
            </p>
          </div>
        </div>
      )}
    </div>
  );
};

export default DebugPanel;
