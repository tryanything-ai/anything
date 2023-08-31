import { useState, useEffect } from "react";
import { useSqlContext } from "../context/SqlProvider";
import { useParams } from "react-router-dom";
import { useNavigationContext } from "../context/NavigationProvider";
import { VscClose } from 'react-icons/vsc'; 

const NodeConfigPanel = () => {
  const { getFlowEvents } = useSqlContext();
  const { nodeId, setNodeConfigPanel} = useNavigationContext(); 
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
      <button className="btn btn-primary" onClick={() => setNodeConfigPanel(false, "")}>
        <VscClose />
      </button>
      {nodeId}
    </div>
  );
};

export default NodeConfigPanel;
