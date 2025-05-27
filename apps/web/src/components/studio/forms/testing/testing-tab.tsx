import { useEffect, useState } from "react";
import { Button } from "@repo/ui/components/ui/button";
import { useAnything } from "@/context/AnythingContext";
import { Play, Loader2 } from "lucide-react";
import { TaskResult } from "./task-card";
import { formatDuration, intervalToDuration } from "date-fns";

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

  // Local state to control minimum testing duration
  const [isTransitioning, setIsTransitioning] = useState(false);
  const [showTestingState, setShowTestingState] = useState(false);

  useEffect(() => {
    if (testingWorkflow) {
      setShowTestingState(true);
      setIsTransitioning(true);
    } else if (isTransitioning) {
      // When testing finishes, wait for minimum duration before hiding the testing state
      const timer = setTimeout(() => {
        setIsTransitioning(false);
        setShowTestingState(false);
      }, 800); // Minimum duration of 800ms for the testing state
      return () => clearTimeout(timer);
    }
  }, [testingWorkflow, isTransitioning]);

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
  }, []);

  return (
    <div className="flex flex-col h-full w-full">
      <div className="">
        <div className="flex flex-row gap-2 items-center">
          <Button
            onClick={runWorkflow}
            className="hover:bg-green-500 transition-all duration-300 min-w-[140px]"
            disabled={testingWorkflow}
          >
            <div className="flex items-center justify-center w-full transition-all duration-300">
              {showTestingState ? (
                <>
                  <span className="opacity-90">Testing...</span>
                  <Loader2 size={16} className="ml-2 animate-spin opacity-90" />
                </>
              ) : (
                <>
                  <span>Test Workflow</span>
                  <Play size={16} className="ml-2" />
                </>
              )}
            </div>
          </Button>
          <div className="relative h-[40px] flex items-center">
            {testStartedTime && (
              <div
                className={`
                  p-2 rounded-lg bg-gray-200 
                  transition-all duration-300 
                  ${testStartedTime ? "opacity-100 translate-y-0" : "opacity-0 translate-y-2"}
                `}
              >
                {testFinishedTime && !isTransitioning
                  ? "Complete"
                  : "Running..."}
              </div>
            )}
            {/* {testStartedTime && (
              <div
                className={`
                  p-2 rounded-lg bg-gray-200 
                  transition-all duration-300 
                  ${testStartedTime ? "opacity-100 translate-y-0" : "opacity-0 translate-y-2"}
                `}
              >
                {testFinishedTime && !isTransitioning
                  ? formatDuration(
                      intervalToDuration({
                        start: new Date(testStartedTime),
                        end: new Date(testFinishedTime),
                      }),
                    )
                  : "Running..."}
              </div>
            )} */}
          </div>
        </div>

        <div className="mt-4 space-y-2">
          {(testingWorkflow || isTransitioning) &&
            worklowTestingSessionTasks.length === 0 && (
              <div className="flex items-center gap-2 text-muted-foreground transition-opacity duration-300">
                <Loader2 size={14} className="animate-spin" />
                <span>Connecting to workflow session...</span>
              </div>
            )}
          <div className="space-y-2 transition-all duration-300">
            {worklowTestingSessionTasks.map((task, index) => (
              <div
                key={index}
                className="transition-all duration-300 animate-in fade-in slide-in-from-bottom-2"
                style={{ animationDelay: `${index * 50}ms` }}
              >
                <TaskResult task={task} getActionIcon={getActionIcon} />
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
