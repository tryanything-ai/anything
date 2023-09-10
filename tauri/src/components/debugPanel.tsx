import { useState, useEffect } from "react";
import { useSqlContext } from "../context/SqlProvider";
import { useParams } from "react-router-dom";
import { useFlowContext } from "../context/FlowProvider";
import clsx from "clsx";
import ReactJson from "react-json-view";
import { formatDistanceToNow } from "date-fns";

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
    <div className="flex flex-col h-full p-4 ">
      {events.length > 0 ? (
        <>
          <div className="text-2xl font-bold">Processing Tasks</div>
          <ul>
            {events.map((event) => (
              <DebugCard key={event.event_id} event={event} />
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
      className={clsx(
        "h-20 w-full my-2 flex flex-col bg-white bg-opacity-5 rounded-md p-2",
        {
          "h-auto": isExpanded,
        }
      )}
    >
      {/* <div onClick={() => setIsExpanded(!isExpanded)}> */}
      <div className="text-2xl">{context?.title}</div>
      {event && event.created_at ? (
        <div className="text-sm">
          {formatDistanceToNow(new Date(event?.created_at), {
            includeSeconds: true,
          })}{" "}
          ago
        </div>
      ) : null}

      {/* <span> {isExpanded ? "▲" : "▼"}</span> */}
    </div>

    // {isExpanded && result && (
    //   <div>
    //     <ReactJson src={result} />
    //   </div>
    // )}
    // </div>
  );
};

export default DebugPanel;
