import { useEffect } from "react";
import { Button } from "@repo/ui/components/ui/button";
import { useAnything } from "@/context/AnythingContext";
import { Play } from "lucide-react";
import { TaskResult } from "./task-card";
import { formatDuration, intervalToDuration } from "date-fns";

export default function TestingTab(): JSX.Element {
  const {
    testing,
    workflow: { getActionIcon, setShowExplorer },
  } = useAnything();

  const runWorkflow = async () => {
    try {
      setShowExplorer(false);
      testing.testWorkflow();
    } catch {
      console.error("Error testing workflow");
    }
  };

  useEffect(() => {
    return () => {
      // Clear any data or state related to the testing workflow when the component unmounts
      testing.resetState();
    };
  }, []);

  return (
    <div className="flex flex-col h-full w-full">
      <div className="">
        <div className="flex flex-row gap-2">
          <Button onClick={runWorkflow} className="hover:bg-green-500">
            Test Workflow
            <Play size={16} className="ml-2" />
          </Button>
          {testing.testFinishedTime ? (
            <div className="p-2 rounded-lg bg-green-200">
              Run Time:{" "}
              {formatDuration(
                intervalToDuration({
                  start: testing.testStartedTime,
                  end: testing.testFinishedTime,
                }),
              )}
            </div>
          ) : null}
        </div>
        {testing.worklowTestingSessionTasks.map((task, index) => (
          <TaskResult key={index} task={task} getActionIcon={getActionIcon} />
        ))}
      </div>
    </div>
  );
}
