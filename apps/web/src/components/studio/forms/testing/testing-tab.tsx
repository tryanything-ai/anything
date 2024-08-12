import { Button } from "@/components/ui/button";
import { useAnything } from "@/context/AnythingContext";
import { Play } from "lucide-react";
import { TaskResult } from "./task-card";
import { formatDuration, intervalToDuration } from "date-fns";

export default function TestingTab() {
  const { workflow, testing } = useAnything();

  const runWorkflow = async () => {
    try {
      testing.testWorkflow();
    } catch {
      console.error("Error testing workflow");
    }
  };

  //TODO: show results of testing.
  //Polling: for results as they happen

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
                })
              )}
            </div>
          ) : null}
        </div>
        {testing.worklowTestingSessionTasks.map((task, index) => (
          <TaskResult key={index} task={task} />
        ))}
      </div>
    </div>
  );
}
