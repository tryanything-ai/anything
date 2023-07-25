import { useTomlFlowContext } from "../context/TomlFlowProvider";

const NodePanel = () => {
  const { addNode } = useTomlFlowContext();
  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500">
      <button onClick={addNode} className="btn btn-neutral">
        Add Node
      </button>
    </div>
  );
};

export default NodePanel;
