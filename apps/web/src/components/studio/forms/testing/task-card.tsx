import React from "react";
import { cn, formatTimeDifference } from "@/lib/utils";
import { formatDuration, intervalToDuration } from "date-fns";
import ReactJson from "react-json-view";
import { TaskRow } from "@/lib/anything-api/testing";
import TaskStatus from "./task-status";
import { Clock } from "lucide-react";
import { Badge } from "@repo/ui/components/ui/badge";

export const TaskResult = React.memo(({ task }: { task: TaskRow }) => {
  return (
    <div
      key={task.task_id}
      className={cn(
        "h-auto w-full my-2 flex flex-col bg-white bg-opacity-5 border rounded-md p-3 text-primary-content",
      )}
    >
      <div className="pb-4">
        <div className="text-xl font-bold">{task.action_label} </div>
      </div>
      <div>
        <TaskStatus
          status={task.task_status}
          started_at={task.started_at ? task.started_at : ""}
          ended_at={task.ended_at}
        />
      </div>
      {/* {task.started_at && task.ended_at && (
        <div>{formatTimeDifference(task.started_at, task.ended_at)}</div>
      )} */}

      {/* <div>Start Time: {task.started_at}</div>
      <div>End Time: {task.ended_at}</div> */}
      {/* {task.started_at && task.ended_at && (
        <div className="m-4 p-2 rounded-lg bg-green-400">
          Run Time:{" "}
          {formatDuration(
            intervalToDuration({
              start: new Date(task.started_at),
              end: new Date(task.ended_at),
            })
          )}
        </div>
      )} */}
      {/* {event.config && (
          <div className="">
            <div className="text-md">Action Config: </div>
            <ResultComponent result={event.config} />
          </div>
        )} */}
      {/* {task.context && (
        <div>
          <div className="text-md">Generated Context: </div>
          <ResultComponent result={task.context} />
        </div>
      )} */}
      {task.result && (
        <div className="my-2">
          <div className="text-md font-semibold">Results: </div>
          <ResultComponent result={task.result} />
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
            style={{
              backgroundColor: "whitesmoke",
              borderRadius: "10px",
              padding: "10px",
            }}
            enableClipboard={false}
            collapsed={1}
            theme={"shapeshifter:inverted"}
            name={false}
            collapseStringsAfterLength={20}
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
            style={{
              backgroundColor: "whitesmoke",
              borderRadius: "10px",
              padding: "10px",
            }}
            collapsed={1}
            collapseStringsAfterLength={20}
            theme={"shapeshifter:inverted"}
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
