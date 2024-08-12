
import { useAnything } from "@/context/AnythingContext";


export default function WorkflowTestingWizard() {
    const { workflow } = useAnything();

    return (
        <div className="grid w-full items-start gap-6">
            Testing Wizard
        </div>
    )
}