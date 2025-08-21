import { useEffect } from "react";
import { Button } from "@repo/ui/components/ui/button";
import { useAnything } from "@/context/AnythingContext";
import { Play, Loader2 } from "lucide-react";
import { TaskResult } from "./task-card";

export default function TestingTab(): JSX.Element {
  const {
    testing: {
      testingWorkflow,
      testWorkflow,
      resetState,
      testFinishedTime,
      testStartedTime,
      worklowTestingSessionTasks,
    },
    workflow: { getActionIcon, setShowExplorer },
  } = useAnything();

  const runWorkflow = async () => {
    try {
      setShowExplorer(false);
      testWorkflow();
    } catch {
      console.error("Error testing workflow");
    }
  };

  useEffect(() => {
    return () => {
      // Clear any data or state related to the testing workflow when the component unmounts
      resetState();
    };
  }, [resetState]);

  return (
    <div className="flex flex-col h-full w-full">
      <div className="">
        <div className="flex flex-row gap-2 items-center">
          <Button
            onClick={runWorkflow}
            className="hover:bg-green-500 transition-all duration-300 min-w-[140px]"
            disabled={testingWorkflow}
          >
            <div className="flex items-center justify-center w-full">
              {testingWorkflow ? (
                <>
                  <span>Testing...</span>
                  <Loader2 size={16} className="ml-2 animate-spin" />
                </>
              ) : (
                <>
                  <span>Test Workflow</span>
                  <Play size={16} className="ml-2" />
                </>
              )}
            </div>
          </Button>
          {testStartedTime && (
            <div className="p-2 rounded-lg bg-gray-200">
              {testFinishedTime ? "Complete" : "Running..."}
            </div>
          )}
        </div>

        <div className="mt-4 space-y-2">
          {testingWorkflow && worklowTestingSessionTasks.length === 0 && (
            <div className="flex items-center gap-2 text-muted-foreground">
              <Loader2 size={14} className="animate-spin" />
              <span>Connecting to workflow session...</span>
            </div>
          )}
          <div className="space-y-2">
            {worklowTestingSessionTasks.map((task, index) => (
              <TaskResult
                key={task.task_id || index}
                task={task}
                getActionIcon={getActionIcon}
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
