import clsx from "clsx";
import { formatDistanceToNow } from "date-fns";
import React, { useEffect, useState } from "react";
import ReactJson from "react-json-view";
import { useParams } from "react-router-dom";
import { v4 as uuidv4 } from "uuid";

import { useFlowContext } from "../context/FlowProvider";
import { VscInfo } from "react-icons/vsc";
import api from "../tauri_api/api";
import { Trigger } from "../utils/flowTypes";

const DebugPanel = () => {
  const { flow_name } = useParams<{ flow_name: string }>();
  const { flowFrontmatter, getTrigger } = useFlowContext();
  const [eventIds, setEventIds] = useState<string[]>([]);
  const { currentProcessingStatus } = useFlowContext();

  const [trigger, setTrigger] = useState<Trigger>(null);
  const [session_id, setSessionId] = useState<string>("");
  const [pinging, setPinging] = useState<boolean>(false);

  const hydrateEvents = async () => {
    try {
      console.log("Hydrating debug panel");
      // console.log("flow_name", flow_name);

      if (!flow_name) {
        console.log("Don't have data needed to hydrate state");
        return;
      }

      // if (!session_id) {
      //   console.log("No session id");
      //   return;
      // }

      console.log("fetching session events: ", session_id);
      //TODO: figure out how to pass types through this barrier
      const newEvents: any = await api.flows.fetchSessionEvents(
        session_id
      )

      console.log("newEvents object", newEvents);

      let events = newEvents.events;

      const newEventIds = events.map((event: any) => event.event_id);

      console.log("Hydrating new eventIds", newEventIds);

      setEventIds(newEventIds);
    } catch (error) {
      console.log("error", error);
    }
  };

  const hydrateMockData = () => {
    try {
      const trigger = getTrigger();
      console.log("found trigger", trigger);
      setTrigger(trigger);
    } catch (error) {
      console.log("error", error);
    }
  }

  const start = async () => {
    try {
      let session_id = uuidv4();
      setSessionId(session_id);
      console.log("session_id from debug panel", session_id);
      let res = await api.flows.executeFlow(
        flowFrontmatter.flow_id,
        flowFrontmatter.flow_version_id,
        session_id, //session_id
        "DEBUG" //stage
      );
      console.log("res from execute flow", res);
      startPinging();
    } catch (error) {
      console.log("error executingFlow from DebugPanel", error);
    }
  };

  const startPinging = () => {
    setPinging(true);
    console.log("start pinging")
    const interval = setInterval(() => {
      if (!pinging) {
        clearInterval(interval);
      } else {
        console.log("pinging");
        hydrateEvents();
      }
    }, 2000);
  };

  const stopPinging = () => {
    setPinging(false);
  };

  useEffect(() => {
    if (currentProcessingStatus) {
      hydrateEvents();
    }
  }, [currentProcessingStatus]);

  useEffect(() => {
    hydrateEvents();
  }, []);

  useEffect(() => {
    hydrateMockData();
  }, [])


  return (
    <div className="flex flex-col gap-4 h-full p-4 overflow-y-auto hide-scrollbar">
      <button className="btn btn-primary hover:btn-success" onClick={start}>
        Start Flow
      </button>
      {/* MockData for Manual Trigger */}
      <div>
        <div className="flex flex-row gap-1">
          Test Inputs
          <div
            className="tooltip tooltip-right"
            data-tip="Test inputs is the shape of the future real inputs from your trigger. It is used for testing."
          >
            <VscInfo />
          </div>
        </div>

        <ReactJson
          style={{ borderRadius: "10px", padding: "10px" }}
          enableClipboard={false}
          theme={"tube"}
          src={trigger ? trigger.mockData : {}}
        />
      </div>
      {/* Event Processiong State */}
      {eventIds.length > 0 ? (
        <div className="text-2xl font-bold">Processing Tasks</div>
      ) : (
        <div className="flex-1">
          <div>
            <h1 className="text-2xl font-bold">No Tasks</h1>
            <p className="">Tasks will appear here when your flow runs</p>
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
  const [label, setLabel] = useState<string>("");
  const [result, setResult] = useState<any>(null);
  const [createdAt, setCreatedAt] = useState<any>(null);


  const getEvent = async (event_id: string) => {
    try {
      const event: any = await api.flows.getEvent(
        event_id
      )
      console.log("event in DebugCard", event);
      // setResult(event.event_result);
      return event.event;
    }
    catch (error) {
      console.log("error", error);
    }

  }


  const hydrate = async () => {
    try {
      // console.log("event_id", event_id);
      const data = await getEvent(event_id);
      console.log("data in DebugCard direct query", data);

      if (data) {
        // if (data?.event_context) {
        setLabel(data?.node_id);
        // }

        if (data?.event_result) {
          setResult(data?.result);
        }

        if (data?.created_at) {
          setCreatedAt(data?.created_at);
        }
      }
    } catch (error) {
      console.log("error", error);
    }
  };

  //   //hydrate own data
  useEffect(() => {
    hydrate();
    const intervalId = setInterval(hydrate, 1000);
    return () => clearInterval(intervalId);
  }, []);

  return (
    <div
      key={event_id}
      className={clsx(
        "h-auto w-full my-2 flex flex-col bg-white bg-opacity-5 rounded-md p-3 text-primary-content"
      )}
    >
      <div className="pb-4">
        <div className="text-xl">{label}</div>
        {createdAt ? (
          <div className="text-xs text-base-content">
            {formatDistanceToNow(new Date(createdAt), {
              includeSeconds: true,
            })}{" "}
            ago
          </div>
        ) : null}
      </div>
      {result && (
        <div className="">
          <div className="text-md">Results: </div>
          <ResultComponent result={result} />
        </div>
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
        content = (
          <ReactJson
            style={{ borderRadius: "10px", padding: "10px" }}
            enableClipboard={false}
            theme={"tube"}
            src={parsedJson}
          />
        );
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
        content = (
          <ReactJson enableClipboard={false} theme={"tube"} src={result} />
        );
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
