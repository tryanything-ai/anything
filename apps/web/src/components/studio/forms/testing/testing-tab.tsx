
import { Button } from "@/components/ui/button";
import { useAnything } from "@/context/AnythingContext";
import { Play } from "lucide-react";


export default function TestingTab() {
    const { workflow, testing } = useAnything();

    const runAction = async () => {
        try {
            testing.testWorkflow();
        } catch {
            console.error("Error testing workflow");
        }
    }

    const runWorkflow = async () => {

    }

    return (
        <div className="grid w-full items-start gap-6">
            <div className="w-full">
                <Button onClick={runAction} className="hover:bg-green-500">
                    Test Workflow
                    <Play size={16} className="ml-2" />
                </Button>
                {testing.testingWorkflow ? "TESTING WORKFLOW" : null}
            </div>
        </div>
    )
}