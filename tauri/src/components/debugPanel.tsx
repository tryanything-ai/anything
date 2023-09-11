import React, { useState, useEffect } from "react";
import { useSqlContext } from "../context/SqlProvider";
import { useParams } from "react-router-dom";
import { useFlowContext } from "../context/FlowProvider";
import clsx from "clsx";
import ReactJson from "react-json-view";
import { formatDistanceToNow } from "date-fns";

const DebugPanel = () => {
  const { getSessionEvents } = useSqlContext();
  const { flow_name } = useParams<{ flow_name: string }>();
  const [eventIds, setEventIds] = useState<string[]>([]);
  const { currentProcessingStatus } = useFlowContext();

  const hydrate = async () => {
    try {
      console.log("Hydrating debug panel");
      console.log("flow_name", flow_name);

      if (!flow_name) {
        console.log("Don't have data needed to hydrate state");
        return;
      }

      if (!currentProcessingStatus?.session_id) {
        console.log("No session id");
        return;
      }

      const newEvents = await getSessionEvents(
        flow_name,
        currentProcessingStatus?.session_id
      );

      const newEventIds = newEvents.map((event: any) => event.event_id);

      console.log("Hydrating new eventIds", newEventIds);

      setEventIds(newEventIds);
    } catch (error) {
      console.log("error", error);
    }
  };

  useEffect(() => {
    if (currentProcessingStatus) {
      hydrate();
    }
  }, [currentProcessingStatus]);

  useEffect(() => {
    hydrate();
  }, []);

  return (
    <div className="flex flex-col h-full p-4 overflow-y-auto">
      {eventIds.length > 0 ? (
        <div className="text-2xl font-bold">Processing Tasks</div>
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
      <ul>
        {eventIds.map((eventId) => (
          <DebugCard key={eventId} event_id={eventId} />
        ))}
      </ul>
    </div>
  );
};

const DebugCard = React.memo(({ event_id }: { event_id: string }) => {
  const [isExpanded, setIsExpanded] = useState(true);
  // const [event, setEvent] = useState<any>(null);
  const [label, setLabel] = useState<string>("");
  const [result, setResult] = useState<any>(null);
  const [createdAt, setCreatedAt] = useState<any>(null);

  const { getEvent } = useSqlContext();

  const hydrate = async () => {
    try {
      // console.log("event_id", event_id);
      const data = await getEvent(event_id);
      console.log("data in DebugCard direct query", data);

      if (data) {
        // if (data?.event_context) {
          setLabel(data?.node_label);
        // }

        if (data?.event_result) {
          setResult(data?.event_result);
        }

        if (data?.created_at) {
          setCreatedAt(data?.created_at);
        }
      }
    } catch (error) {
      console.log("error", error);
    }
  };

  //hydrate own data
  useEffect(() => {
    hydrate();
    const intervalId = setInterval(hydrate, 1000);
    return () => clearInterval(intervalId);
  }, []);

  return (
    <div
      key={event_id}
      className={clsx(
        "h-20 w-full my-2 flex flex-col bg-white bg-opacity-5 rounded-md p-2 ",
        {
          "h-auto": isExpanded,
        }
      )}
    >
      <div className="text-2xl">{label}</div>
      {createdAt ? (
        <div className="text-xsm">
          {formatDistanceToNow(new Date(createdAt), {
            includeSeconds: true,
          })}
          ago
        </div>
      ) : null}
      {result ? (
        <div onClick={() => setIsExpanded(!isExpanded)}>
          {isExpanded ? "Hide Results" : "View Results"}
        </div>
      ) : null}
      {isExpanded && result && (
        <>
          <ResultComponent result={result} />
          {/* {JSON.stringify(result, null, 2)} */}
        </>
      )}
    </div>
  );
});

const ResultComponent = ({ result }: any) => {
  let content;

  switch (typeof result) {
    case "string":
      try {
        const parsedJson = JSON.parse(result);
        console.log("parsedJson", parsedJson);
        content = <ReactJson theme={"monokai"} src={parsedJson} />;
      } catch (e) {
        content = result;
      }
      break;
    case "number":
      content = result.toString();
      break;
    case "object":
      if (result !== null) {
        console.log("result in object switch", result);
        content = <ReactJson theme={"monokai"} src={result} />;
      } else {
        content = "null";
      }
      break;
    default:
      content = "Unsupported type";
  }

  return <div>{content}</div>;
};
export default DebugPanel;
