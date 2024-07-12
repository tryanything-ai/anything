import { useState } from "react";
import { useAnything } from "@/context/AnythingContext";
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"
import { SubmitHandler, useForm } from "react-hook-form";
import { useRouter } from "next/navigation";
import DeleteFlowDialog from "./delete-flow-dialog";
import { Switch } from "@/components/ui/switch";

type Inputs = {
    flow_name: string;
};

export default function SettingsForm() {
    const { workflow } = useAnything();
    const [loading, setLoading] = useState(false);
    const navigate = useRouter();

    const {
        register,
        handleSubmit,
        formState: { errors },
    } = useForm<Inputs>();

    // const onSubmit: SubmitHandler<Inputs> = async (data) => {
    //     try {
    //         setLoading(true);
    //         if (flow_name && flowFrontmatter) {
    //             let UpdateFlowArgs = {
    //                 flow_name: data.flow_name,
    //                 active: flowFrontmatter.active,
    //                 version: flowFrontmatter.version,
    //             };

    //             console.log(
    //                 "Updating Flow In Settings Panel with Args",
    //                 UpdateFlowArgs
    //             );
    //             let res = await updateFlow(flowFrontmatter.flow_id, UpdateFlowArgs);
    //             console.log("res from rename flow in settings panel", res);
    //             // navigate(`/flows/${data.flow_name}`);
    //             navigate.back();
    //         } else {
    //             console.log("Data problem in settings panel");
    //         }
    //     } catch (error) {
    //         console.log("error in settings panel", error);
    //     } finally {
    //         console.log(data);
    //         setLoading(false);
    //     }
    // };

    return (
        <form className="grid w-full items-start gap-6">
            <fieldset className="grid gap-6 rounded-lg border p-4">
                <div className="grid gap-3">
                    <Label htmlFor="model">Workflow Name</Label>
                    <Input id="model"
                        type="text"
                        placeholder="Workflow Name"
                        className="input input-bordered input-md w-full"
                        defaultValue={workflow.db_flow?.flow_name}
                        {...register("flow_name", { required: true })}
                    />
                </div>
            </fieldset>
            <div className="flex flex-row justify-between m-4">
                <div>Detailed Editor</div>
                <Switch
                    checked={workflow.detailedMode}
                    onCheckedChange={() => workflow.setDetailedMode(!workflow.detailedMode)}
                />
            </div>

            <div className="absolute bottom-0 w-full mb-2">
                <DeleteFlowDialog workflowId={workflow.db_flow_id} />
            </div>
        </form>
    )
}