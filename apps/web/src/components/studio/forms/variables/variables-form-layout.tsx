import InputVariablesForm from "./input-variables-form";
import { useAnything } from "@/context/AnythingContext";
import { EditVariableFormMode } from "@/context/VariablesContext";
import EditVariableForm from "./edit-variable-form";
import EditVariablesForm from "./edit-variables-form";
import { Button } from "@repo/ui/components/ui/button";
import { ActionType } from "@/types/workflows";
import { Lock, ChevronRight } from "lucide-react";
import { useState } from "react";

export function VariablesFormLayout(): JSX.Element {
  const { variables, workflow } = useAnything();
  const [isCollapsed, setIsCollapsed] = useState(false);

  const Header = () => {
    let header_title = "Variables";
    let link_button_text = "";
    let action = () => {};
    let variables_locked = workflow?.selected_node_data?.variables_locked;
    let variables_schema_locked =
      workflow?.selected_node_data?.variables_schema_locked;

    switch (variables.editingMode) {
      case EditVariableFormMode.EDIT:
        header_title = "Edit Variable";
        link_button_text = "Cancel";
        action = () => variables.setEditingMode(EditVariableFormMode.INPUT);
        break;
      case EditVariableFormMode.DELETE:
        header_title = "Edit Variables";
        link_button_text = "Cancel";
        action = () => variables.setEditingMode(EditVariableFormMode.INPUT);
        break;
      case EditVariableFormMode.INPUT:
        header_title = "Variables";
        link_button_text =
          Object.keys(
            workflow?.selected_node_variables_schema?.properties || {},
          ).length > 0
            ? "Edit"
            : "Add New Variable";
        action = () => variables.setEditingMode(EditVariableFormMode.DELETE);
        break;
      default:
        header_title = "Variables";
    }

    return (
      <div className="flex flex-row items-center">
        <Button
          variant="ghost"
          size="sm"
          className="mr-2"
          onClick={() => setIsCollapsed(!isCollapsed)}
        >
          <ChevronRight
            className={`h-4 w-4 transition-transform ${!isCollapsed ? "rotate-90" : ""}`}
          />
        </Button>
        <div className="font-bold">{header_title}</div>
        <div className="flex-1" />
        {!variables_schema_locked ? (
          <Button variant={"link"} onClick={action}>
            {link_button_text}
          </Button>
        ) : (
          // <Lock size={16} className="text-gray-400" />
          <></>
        )}
      </div>
    );
  };

  const renderEditor = () => {
    switch (variables.editingMode) {
      case EditVariableFormMode.EDIT:
        return <EditVariableForm />;
      case EditVariableFormMode.DELETE:
        return <EditVariablesForm />;
      case EditVariableFormMode.INPUT:
        return <InputVariablesForm />;
      default:
        null;
    }
  };

  return (
    <>
      {workflow && workflow.selected_node_data && (
        <div className="rounded-lg border p-4">
          <Header />
          {!isCollapsed && renderEditor()}
        </div>
      )}
    </>
  );
}
