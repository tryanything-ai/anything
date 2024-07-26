
import { Button } from "@/components/ui/button";
import { useAnything } from "@/context/AnythingContext";
import { Play } from "lucide-react";
import { TaskResult } from "./task-card";

export default function TestingTab() {
    const { workflow, testing } = useAnything();

    const runWorkflow = async () => {
        try {
            testing.testWorkflow();
        } catch {
            console.error("Error testing workflow");
        }
    }

    //TODO: show results of testing. 
    //Polling: for results as they happen

    return (
        <div className="grid w-full items-start gap-6">
            <div className="w-full">
                <Button onClick={runWorkflow} className="hover:bg-green-500">
                    Test Workflow
                    <Play size={16} className="ml-2" />
                </Button>
                {testing.testingWorkflow ? "TESTING WORKFLOW" : null}
                {testing.worklowTestingSessionTasks.map((task, index) => (
                    <TaskResult key={index} task={task} />
                ))}
            </div>
        </div>
    )
}