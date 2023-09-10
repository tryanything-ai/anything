import { useState, useEffect } from "react";
import { useSqlContext } from "../context/SqlProvider";
import { useParams } from "react-router-dom";
import { useFlowContext } from "../context/FlowProvider";
import clsx from "clsx";

const DebugPanel = () => {
  const { getFlowEvents } = useSqlContext();
  const { flow_name } = useParams();
  const [events, setEvents] = useState<any[]>([]);
  const { currentProcessingStatus } = useFlowContext();

  const hydrate = async () => {
    try {
      if (!flow_name) return;
      if (!currentProcessingStatus?.session_id) {
        console.log("no session id in debug panel");
        return;
      }

      const data = await getFlowEvents(
        flow_name,
        currentProcessingStatus.session_id
      );

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
      {events.length > 0 ? (
        <>
          <div className="text-2xl font-bold">Processing Tasks</div>
          <ul>
            {events.map((event) => (
              <DebugCard key={event.id} event={event} />
            ))}
          </ul>
        </>
      ) : (
        <div className="flex-1 text-center">
          <div>
            <h1 className="text-2xl font-bold">No Tasks</h1>
            <p className="text-sm p-2">
              Tasks will appear here when your flow runs
            </p>
          </div>
        </div>
      )}
    </div>
  );
};

const DebugCard = ({ event }: { event: any }) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const context = JSON.parse(event?.event_context);
  const result = JSON.parse(event?.event_result);

  return (
    <div
      key={event.event_id}
      className={clsx("card h-20 w-full text-md border p-4 my-2", {
        "h-auto": isExpanded,
      })}
    >
      <div onClick={() => setIsExpanded(!isExpanded)}>
        <span>{context?.title}</span>
        <span> {isExpanded ? "▲" : "▼"}</span>
      </div>

      {isExpanded && (
        <div>
          <pre>{JSON.stringify(result, null, 2)}</pre>
        </div>
      )}
    </div>
  );
};

export default DebugPanel;
