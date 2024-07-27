import React from "react";
import {cn } from "@/lib/utils";
import { formatDistanceToNow } from "date-fns";
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
          <div className="text-xl">{task.processing_order + 1}:{" "}{task.node_id}</div>
          {task.created_at ? (
            <div className="text-xs text-base-content">
              {formatDistanceToNow(new Date(task.created_at), {
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