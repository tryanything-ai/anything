import { useAnything } from "@/context/AnythingContext";
import { ActionType } from "@/types/workflows";

export function ResultsExplorer(): JSX.Element {
  const { workflow } = useAnything();

  const Header = () => {
    let header_title = "Action Results";

    return (
      <div className="flex flex-row items-center">
        <div className="font-bold">{header_title}</div>
        <div className="flex-1" />
      </div>
    );
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
          </div>
        )}
    </>
  );
}
