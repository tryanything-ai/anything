import { useTomlFlowContext } from "../context/TomlFlowProvider";

const ChatPanel = () => {
  //   const { addNode } = useTomlFlowContext();
  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500">
      <button className="btn btn-neutral">CHAT</button>
    </div>
  );
};

export default ChatPanel;
