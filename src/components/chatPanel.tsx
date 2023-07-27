import { randomUUID } from "crypto";
import { useSqlContext } from "../context/SqlProvider";

const ChatPanel = () => {
  const { addEvent } = useSqlContext();

  const createMockEvent = () => {
    addEvent(
      randomUUID(),
      "flow_id",
      "flow_name",
      "flow_version",
      "stage",
      "status",
      "created_at",
      "data"
    );
  };

  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500">
      <button className="btn btn-neutral">Chat</button>
      <button onClick={createMockEvent} className="btn btn-neutral">
        Add Event
      </button>
    </div>
  );
};

export default ChatPanel;
