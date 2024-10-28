import { useAnything } from "@/context/AnythingContext";
import { ActionType } from "@/types/workflows";
import { useState } from "react";
import { Button } from "@repo/ui/components/ui/button";

export function ResultsExplorer(): JSX.Element {
  const {
    workflow,
    explorer: { insertAtCursor },
  } = useAnything();

  const [results, setResults] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);

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
    <div className="grid w-full items-start gap-6 p-2">
      {" "}
      {workflow &&
        workflow.selected_node_data &&
        workflow.selected_node_data.type !== ActionType.Trigger && (
          <div className="rounded-lg border p-4">
            <Header />
            {loading && <div>Loading...</div>}
            {results.length === 0 && !loading && (
              <div className="text-muted-foreground">
                Run Workflow Test Access Results
              </div>
            )}
            <Button
              onClick={() =>
                insertAtCursor(
                  "{{actions.results.body.hello_world}}",   // Or whatever field name you're targeting
                )
              }
            >
              Insert Template
            </Button>
          </div>
        )}
    </div>
  );
}
