import { useState, useEffect } from "react";
import { useFlowContext } from "../context/FlowProvider";
import { useSqlContext, EventInput } from "../context/SqlProvider";
import { useParams } from "react-router-dom";

const DebugPanel = () => {
  const { getFlowEvents } = useSqlContext();
  const { flow_name } = useParams();
  const [events, setEvents] = useState<any[]>([]);

  const hydrate = async () => {
    try {
      if (!flow_name) return;
      const data = await getFlowEvents(flow_name);
      console.log("data in debug Panel", data);
      setEvents(data);
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

  // const createMockEvent = () => {
  //   console.log("createMockEvent");

  //   if (flow_name === undefined) return;
  //   //TODO: make unique flow_id's so name changes don't fuck up processing side
  //   //TODO: node_name might also be nice cause then users can see names not id's in TOML
  //   let event: EventInput = {
  //     flow_id: flowFrontmatter.id, //flow_id
  //     flow_name: flow_name,
  //     flow_version: "0.0.1",
  //     node_id: "node_id",
  //     node_type: "manualNode", //node type, lets the machine know it should boostrap the
  //     worker_type: "start",
  //     stage: "dev",
  //     event_status: "PENDING", //EVENT STATUS
  //     session_status: "PENDING", //SESSION STATUS
  //     created_at: new Date().toISOString(),
  //     data: { test: true },
  //   };
  //   addEvent(event);
  // };

  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500">
      {/* <button onClick={createMockEvent} className="btn btn-neutral">
        Add Event
      </button> */}
      {/* <div> */}
      {events.length > 0 ? (
        <ul>
          {events.map((event) => {
            return (
              <div className="card h-20 w-full text-md text-primary-content border p-4 my-2">
                {event.node_type}
              </div>
            );
          })}
        </ul>
      ) : (
        <div className="flex-1 text-center">
          <div>
            <h1 className="text-2xl">No Events</h1>
            <p className="text-sm p-2">
              Events will appear here when you run your flow
            </p>
          </div>
        </div>
      )}
      {/* </div> */}
    </div>
  );
};

export default DebugPanel;
