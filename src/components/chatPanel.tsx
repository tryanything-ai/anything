import { useSqlContext } from "../context/SqlProvider";
import { useParams } from "react-router-dom";

const ChatPanel = () => {
  const { addEvent } = useSqlContext();
  const { flow_name } = useParams();

  const createMockEvent = () => {
    console.log("createMockEvent");

    if (flow_name === undefined) return;

    addEvent(
      "1",
      flow_name,
      "0.0.1",
      "dev",
      "PENDING", //COMPLETE, ERROR, PENDING
      new Date().toISOString(),
      { test: true }
    );
  };

  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500">
      <button onClick={createMockEvent} className="btn btn-neutral">
        Add Event
      </button>
    </div>
  );
};

export default ChatPanel;
