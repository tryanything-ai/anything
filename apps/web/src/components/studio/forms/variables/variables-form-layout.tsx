import InputVariablesForm from "./input-variables-form";
import { useAnything } from "@/context/AnythingContext";
import { EditVariableFormMode } from "@/context/VariablesContext";
import EditVariableForm from "./edit-variable-form";
import EditVariablesForm from "./edit-variables-form";
import { Button } from "@repo/ui/components/ui//button";
import { ActionType } from "@/types/workflows";

export function VariablesFormLayout() {
  const { variables, workflow } = useAnything();

  const Header = () => {
    let header_title = "Variables";
    let link_button_text = "";
    let action = () => {};

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
          Object.keys(workflow.selected_node_variables).length > 0
            ? "Edit"
            : "Add New Variable";
        action = () => variables.setEditingMode(EditVariableFormMode.DELETE);
        break;
      default:
        header_title = "Variables";
    }

    return (
      <div className="flex flex-row items-center">
        <div className="font-bold">{header_title}</div>
        <div className="flex-1" />
        <Button variant={"link"} onClick={action}>
          {link_button_text}
        </Button>
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
    // Hide variables if its a trigger
    <>
      {" "}
      {workflow &&
        workflow.selected_node_data &&
        workflow.selected_node_data.type !== ActionType.Trigger && (
          <div className="rounded-lg border p-4">
            <Header />
            {renderEditor()}
          </div>
        )}
    </>
  );
}
