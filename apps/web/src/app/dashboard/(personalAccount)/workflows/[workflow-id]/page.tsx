import { PartyPopper } from "lucide-react";

export default function WorkflowManager() {
    return (
        <div className="flex flex-col gap-y-4 py-12 h-full w-full items-center justify-center content-center max-w-screen-md mx-auto text-center">
            <PartyPopper className="h-12 w-12 text-gray-400" />
            <h1 className="text-2xl font-bold">A single Workflow</h1>

        </div>
    )
}