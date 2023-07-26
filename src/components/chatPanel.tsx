import { useSqlContext } from "../context/SqlProvider";

const ChatPanel = () => {
  const { addEvent } = useSqlContext();

  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500">
      <button className="btn btn-neutral">Chat</button>
      <button onClick={addEvent} className="btn btn-neutral">
        Add Event
      </button>
    </div>
  );
};

export default ChatPanel;
