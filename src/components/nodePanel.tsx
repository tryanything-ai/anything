import { useFlowContext } from "../context/FlowProvider";

const NodePanel = () => {
  const { addNode } = useFlowContext();

  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500">
      <button onClick={() => addNode("default")} className="btn btn-neutral">
        Add Node
      </button>
    </div>
  );
};

export default NodePanel;
