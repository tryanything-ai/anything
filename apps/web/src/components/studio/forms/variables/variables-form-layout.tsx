import InputVariablesForm from "./input-variables-form";
import { useAnything } from "@/context/AnythingContext";
import { EditVariableFormMode } from "@/context/VariablesContext";
import CreateVariableForm from "./create-variable-form";
import EditVariablesForm from "./edit-variables-form";
import { Button } from "@repo/ui/components/ui/button";
import { ChevronRight } from "lucide-react";

export function VariablesFormLayout(): JSX.Element {
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
      <div
        className="flex flex-row items-center cursor-pointer"
        onClick={(e) => {
          // Prevent click from bubbling to parent elements
          e.stopPropagation();
          variables.setIsFormVisible(!variables.isFormVisible);
        }}
      >
        <ChevronRight
          className={`h-4 w-4 transition-transform mr-2 ${
            variables.isFormVisible ? "rotate-90" : ""
          }`}
        />
        <div className="font-bold">{header_title}</div>
        <div className="flex-1" />
        {!workflow?.selected_node_data?.variables_schema_locked ? (
          <Button
            variant={"link"}
            className="h-auto p-0"
            onClick={(e) => {
              e.stopPropagation(); // Prevent collapse/expand when clicking link
              action();
            }}
          >
            {link_button_text}
          </Button>
        ) : (
          // <Lock size={16} className="text-gray-400" />
          <></> // TODO: Figure better ui pattern to show that you can't add variables to locked schemas
        )}
      </div>
    );
  };

  const renderEditor = () => {
    switch (variables.editingMode) {
      case EditVariableFormMode.EDIT:
        return <CreateVariableForm />;
      case EditVariableFormMode.DELETE:
        return <EditVariablesForm />;
      case EditVariableFormMode.INPUT:
        return <InputVariablesForm />;
      default:
        return null;
    }
  };

  return (
    <>
      {workflow && workflow.selected_node_data && (
        <div className="rounded-lg border p-4 w-full overflow-hidden">
          <Header />
          <div className="w-full overflow-hidden">
            {variables.isFormVisible && renderEditor()}
          </div>
        </div>
      )}
    </>
  );
}
