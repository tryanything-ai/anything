
import { useAnything } from "@/context/AnythingContext";


export default function WorkflowTestingWizard(): JSX.Element {
    const { workflow } = useAnything();

    return (
        <div className="grid w-full items-start gap-6">
            Testing Wizard
        </div>
    )
}