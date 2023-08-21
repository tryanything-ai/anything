import { useFlowContext } from "../context/FlowProvider";
import { useSqlContext, EventInput } from "../context/SqlProvider";
import { useParams } from "react-router-dom";

const DebugPanel = () => {
  const { addEvent } = useSqlContext();
  const { flow_name } = useParams();
  const { flowFrontmatter } = useFlowContext();

  const createMockEvent = () => {
    console.log("createMockEvent");

    if (flow_name === undefined) return;
    //TODO: make unique flow_id's so name changes don't fuck up processing side
    //TODO: node_name might also be nice cause then users can see names not id's in TOML
    let event: EventInput = {
      flow_id: flowFrontmatter.id, //flow_id
      flow_name: flow_name,
      flow_version: "0.0.1",
      node_id: "node_id",
      node_type: "manualNode", //node type, lets the machine know it should boostrap the
      worker_type: "start",
      stage: "dev",
      event_status: "PENDING", //EVENT STATUS
      session_status: "PENDING", //SESSION STATUS
      created_at: new Date().toISOString(),
      data: { test: true },
    };
    addEvent(event);
  };

  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500">
      <button onClick={createMockEvent} className="btn btn-neutral">
        Add Event
      </button>
    </div>
  );
};

export default DebugPanel;
