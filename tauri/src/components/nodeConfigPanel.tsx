import { useState, useEffect } from "react";
import { useSqlContext } from "../context/SqlProvider";
import { useParams } from "react-router-dom";
import { useNavigationContext } from "../context/NavigationProvider";
import { VscClose } from "react-icons/vsc";

const NodeConfigPanel = () => {
  const { getFlowEvents } = useSqlContext();
  const { nodeId, setNodeConfigPanel } = useNavigationContext();
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
    <div className="flex flex-col h-full border-l border-gray-500">
      <button
        className="m-1 btn btn-ghost btn-square btn-xs w-6 h-6 absolute right-0"
        onClick={() => setNodeConfigPanel(false, "")}
      >
        <VscClose className="h-6 w-6" />
      </button>
      {nodeId}
    </div>
  );
};

export default NodeConfigPanel;
