import React from "react";
import {cn } from "@/lib/utils";
import { formatDuration, intervalToDuration } from "date-fns";
import ReactJson from "react-json-view";
import { TaskRow } from "@/lib/anything-api/testing";

export const TaskResult = React.memo(({ task }: { task: TaskRow }) => {
    return (
      <div
        key={task.task_id}
        className={cn(
          "h-auto w-full my-2 flex flex-col bg-white bg-opacity-5 border rounded-md p-3 text-primary-content"
        )}
      >
        <div className="pb-4">
          <div className="text-xl">{task.processing_order + 1}:{" "}{task.node_id} </div>
        </div>
        <div>
          Start Time: {task.started_at}
        </div>
        <div>
          End Time: {task.ended_at}
        </div>
        { task.started_at && task.ended_at && <div className="m-4 p-2 rounded-lg bg-green-400">Run Time: {formatDuration(intervalToDuration({start: new Date(task.started_at), end: new Date(task.ended_at)}))}</div> }
        {/* {event.config && (
          <div className="">
            <div className="text-md">Action Config: </div>
            <ResultComponent result={event.config} />
          </div>
        )} */}
        {task.context && (
          <div>
            <div className="text-md">Generated Context: </div>
            <ResultComponent
              result={task.context} />
          </div>
        )}
        {task.result && (
          <div className="">
            <div className="text-md">Results: </div>
            <ResultComponent
              result={task.result} />
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
              name={false}
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
            <ReactJson
              enableClipboard={false}
              name={false}
              style={{ borderRadius: "10px", padding: "10px" }}
              theme={"tube"}
              src={result}
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