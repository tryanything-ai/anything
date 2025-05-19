import React from "react";
import { cn, formatTimeDifference } from "@/lib/utils";
import { formatDuration, intervalToDuration } from "date-fns";
// import ReactJson from "react-json-view";
import { TaskRow } from "@repo/anything-api";
import TaskStatus from "./task-status";

import dynamic from "next/dynamic";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@repo/ui/components/ui/tabs";
import { SvgRenderer } from "../../nodes/node-icon";
import { Check, Copy } from "lucide-react";
import { useState } from "react";

// Dynamically import ReactJson with SSR disabled
const ReactJson = dynamic(() => import("react-json-view"), {
  ssr: false,
  loading: () => <div>Loading...</div>,
});

function TaskIdCopy({ task_id }: { task_id: string }) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(task_id);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  return (
    <div className="flex items-center space-x-2 mt-1 text-xs text-gray-500 select-none group">
      <span className="font-mono bg-gray-100 px-2 py-0.5 rounded text-gray-700 border border-gray-200">
        {task_id}
      </span>
      <button
        onClick={handleCopy}
        className="hover:text-primary focus:outline-none flex items-center transition-opacity opacity-0 group-hover:opacity-100"
        aria-label="Copy Task ID"
        type="button"
      >
        {copied ? (
          <Check className="text-green-500 w-4 h-4" />
        ) : (
          <Copy className="w-4 h-4" />
        )}
      </button>
    </div>
  );
}

export const TaskResult = React.memo(
  ({
    task,
    getActionIcon,
  }: {
    task: TaskRow;
    getActionIcon: (action_id: string) => string;
  }) => {
    return (
      <div
        key={task.task_id}
        className={cn(
          "h-auto w-full my-2 flex flex-col bg-white bg-opacity-5 border rounded-md p-3 text-primary-content",
        )}
      >
        <div className="pb-2">
          <div className="text-xl font-bold flex flex-row items-center">
            <div className="h-6 w-6 mr-2">
              <SvgRenderer icon={getActionIcon(task.action_id)} />
            </div>
            {task.action_label}
          </div>
          <TaskIdCopy task_id={task.task_id} />
        </div>
        {(task.result || task.error || task.context) && (
          <div className="">
            <Tabs
              defaultValue={task.error ? "error" : "result"}
              className="w-full"
            >
              <div className="flex items-center">
                <TabsList className="mr-2">
                  {task.error ? (
                    <TabsTrigger value="error">Error</TabsTrigger>
                  ) : (
                    <TabsTrigger value="result">Results</TabsTrigger>
                  )}
                  <TabsTrigger value="context">Config</TabsTrigger>
                </TabsList>
                <TaskStatus
                  status={task.task_status}
                  started_at={task.started_at ? task.started_at : ""}
                  ended_at={task.ended_at}
                />
              </div>
              {task.error ? (
                <TabsContent value="error">
                  <div>
                    <ResultComponent
                      result={task.error}
                      collapseStringsAfterLength={20}
                    />
                  </div>
                </TabsContent>
              ) : (
                <TabsContent value="result">
                  {task.result && (
                    <div>
                      <ResultComponent
                        result={task.result}
                        collapseStringsAfterLength={20}
                      />
                    </div>
                  )}
                </TabsContent>
              )}
              <TabsContent value="context">
                {task.context && (
                  <div>
                    <ResultComponent
                      result={task.context}
                      collapseStringsAfterLength={20}
                    />
                  </div>
                )}
              </TabsContent>
            </Tabs>
          </div>
        )}
      </div>
    );
  },
);

export const ResultComponent = ({
  result,
  className = "",
  collapseStringsAfterLength,
  collapsed = 1,
}: any) => {
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
            collapsed={collapsed}
            theme={"shapeshifter:inverted"}
            name={false}
            collapseStringsAfterLength={collapseStringsAfterLength}
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
            collapsed={collapsed}
            collapseStringsAfterLength={collapseStringsAfterLength}
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

  return (
    <div className={className} suppressHydrationWarning>
      {content}
    </div>
  );
};
