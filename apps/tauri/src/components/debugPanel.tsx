import clsx from "clsx";
import { formatDistanceToNow } from "date-fns";
import React, { useEffect, useState } from "react";
import ReactJson from "react-json-view";

import { useFlowContext } from "../context/FlowProvider";
import { VscInfo } from "react-icons/vsc";
import { Trigger } from "../utils/flowTypes";
import { DebuggingProvider, SessionStatus, useDebuggingContext } from "../context/DebuggingProvider";

const Debugger = () => {

  const { getTrigger } = useFlowContext();
  const { startDebuggingSession, debugging, events, session_status } = useDebuggingContext();

  const [trigger, setTrigger] = useState<Trigger>(null);

  const hydrateMockData = () => {
    try {
      const trigger = getTrigger();
      console.log("found trigger", trigger);
      setTrigger(trigger);
    } catch (error) {
      console.log("error", error);
    }
  }

  useEffect(() => {
    hydrateMockData();
  }, [])

  interface SessionButtonProps {
    status: SessionStatus;
  }

  const SessionButton: React.FC<SessionButtonProps> = ({ status }) => {
    const renderButton = () => {
      switch (status) {
        case SessionStatus.WAITING:
          return (< button className="btn btn-primary hover:btn-success" disabled onClick={startDebuggingSession} >
            Waiting...
          </button >);
        case SessionStatus.PROCESSING:
          return (< button className="btn btn-primary hover:btn-success" disabled onClick={startDebuggingSession} >
            Processing...
          </button >);
        default:
          return (
            < button className="btn btn-primary hover:btn-success" onClick={startDebuggingSession} >
              Start Flow
            </button >
          );
      }
    };
    return <>{renderButton()}</>;
  };


  return (
    <div className="flex flex-col gap-4 h-full p-4 overflow-y-auto hide-scrollbar max-h-screen">
      <div className="mb-20 flex flex-col gap-4 p-4">
        <SessionButton status={session_status} />

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
        {debugging ? (
          <>
            <div className="text-2xl font-bold">Processing Tasks</div>
            <ul>
              {events.map((event) => (
                <DebugCard key={event.event_id} event={event} />
              ))}
            </ul>
          </>
        ) : (

          <div className="flex-1">
            <div>
              <h1 className="text-2xl font-bold">No Tasks</h1>
              <p className="">Tasks will appear here when your flow runs</p>
            </div>
          </div>

        )}
      </div>
    </div>
  );
};

const DebugCard = React.memo(({ event }: { event: any }) => {
  return (
    <div
      key={event.event_id}
      className={clsx(
        "h-auto w-full my-2 flex flex-col bg-white bg-opacity-5 rounded-md p-3 text-primary-content"
      )}
    >
      <div className="pb-4">
        <div className="text-xl">{event.node_id}</div>
        {event.created_at ? (
          <div className="text-xs text-base-content">
            {formatDistanceToNow(new Date(event.created_at), {
              includeSeconds: true,
            })}{" "}
            ago
          </div>
        ) : null}
      </div>
      {/* {event.config && (
        <div className="">
          <div className="text-md">Action Config: </div>
          <ResultComponent result={event.config} />
        </div>
      )} */}
      {event.context && (
        <div>
          <div className="text-md">Generated Context: </div>
          <ResultComponent
            result={event.context} />
        </div>
      )}
      {event.result && (
        <div className="">
          <div className="text-md">Results: </div>
          <ResultComponent
            result={event.result} />
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
        console.log("result in result ", result);
        content = (
          <ReactJson enableClipboard={false}
            style={{ borderRadius: "10px", padding: "10px" }}
            theme={"tube"} src={result}
          />
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


export default function DebugPanel() {
  return (
    <DebuggingProvider>
      <Debugger />
    </DebuggingProvider>
  );
}
