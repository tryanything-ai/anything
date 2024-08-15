import { useAnything } from "@/context/AnythingContext";
import ActionDisplayForm from "./action-display-form";
import NoNodeSelected from "../no-node-selected";

export default function ActionDisplayTab(): JSX.Element {
  const { workflow } = useAnything();

  return (
    <div className="flex flex-col h-full w-full">
      {workflow.selected_node_data && workflow.selected_node_id ? (
        <div className="grid w-full items-start gap-6">
          <div className="grid gap-6">
            <ActionDisplayForm />
          </div>
        </div>
      ) : (
        <NoNodeSelected />
      )}
    </div>
  );
}
