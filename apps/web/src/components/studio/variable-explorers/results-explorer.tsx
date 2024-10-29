import { useAnything } from "@/context/AnythingContext";
import { ActionType } from "@/types/workflows";
import { useEffect, useState } from "react";
import { Button } from "@repo/ui/components/ui/button";
import api, { TaskRow } from "@repo/anything-api";
import { useAccounts } from "@/context/AccountsContext";

export function ResultsExplorer(): JSX.Element {
  const {
    workflow: { db_flow_id, db_flow_version_id, selected_node_data },
    explorer: { insertAtCursor },
  } = useAnything();

  const [results, setResults] = useState<TaskRow[]>([]);
  const [loading, setLoading] = useState(false);
  const { selectedAccount } = useAccounts();
  const fetchResults = async () => {
    try {
      if (!db_flow_id || !db_flow_version_id || !selectedAccount) {
        console.log("[RESULTS EXPLORER] Missing required IDs:", {
          db_flow_id,
          db_flow_version_id,
          account: selectedAccount?.account_id
        });
        return;
      }

      console.log("[RESULTS EXPLORER] Fetching variables for:", {
        account_id: selectedAccount.account_id,
        flow_id: db_flow_id,
        version_id: db_flow_version_id
      });

      const result = await api.variables.getWorkflowVersionVariables(
        selectedAccount.account_id,
        db_flow_id,
        db_flow_version_id,
      );

      if (result) {
        console.log("[RESULTS EXPLORER] Successfully fetched results:", result);
        setResults(result.tasks);
      } else {
        console.error("[RESULTS EXPLORER] No results found");
      }
    } catch (error) {
      console.error("[RESULTS EXPLORER] Error fetching results:", error);
    }
  };

  useEffect(() => {
    console.log("[RESULTS EXPLORER] Initial fetch triggered");
    fetchResults();
  }, []);

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
      {selected_node_data && selected_node_data.type !== ActionType.Trigger && (
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
                "{{actions.results.body.hello_world}}", // Or whatever field name you're targeting
              )
            }
          >
            Insert Template
          </Button>

          {results.map((task: TaskRow) => (
            <div key={task.task_id} className="flex flex-row items-center">
              <div className="flex-1">{task.task_id}</div>
              <div className="flex-1">{task.action_label}</div>
              <div className="flex-1">{task.task_status}</div>
              <div className="flex-1">
                {JSON.stringify(task.result, null, 3)}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
