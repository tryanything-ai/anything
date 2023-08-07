import { useFlowContext } from "../context/FlowProvider";

export const Nodes = [{}];
const NodePanel = () => {
  const { addNode } = useFlowContext();

  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500">
      <button
        onClick={() => addNode("default")}
        className="btn btn-neutral text-xl"
      >
        Add Node
      </button>
      <button
        onClick={() => addNode("pythonNode")}
        className="btn btn-neutral mt-2 pb-2"
      >
        <img
          src={"/python-logo.svg"}
          alt="Python Logo"
          className="max-w-full max-h-full mt-2 ml-4"
        />
      </button>
      <button
        onClick={() => addNode("javascriptNode")}
        className="btn btn-neutral mt-2 pb-2"
      >
        <img
          src={"/js-logo.svg"}
          alt="JS Logo"
          className="max-w-full max-h-full mt-1"
        />
      </button>
    </div>
  );
};

export default NodePanel;
