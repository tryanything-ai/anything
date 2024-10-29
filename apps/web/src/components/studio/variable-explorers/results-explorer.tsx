import { useAnything } from "@/context/AnythingContext";
import { ActionType } from "@/types/workflows";
import { useEffect, useState } from "react";
import { Button } from "@repo/ui/components/ui/button";
import api, { TaskRow } from "@repo/anything-api";
import { useAccounts } from "@/context/AccountsContext";
import { JsonExplorer } from "./json-explorer";
import { ScrollArea } from "@repo/ui/components/ui/scroll-area";

export function ResultsExplorer(): JSX.Element {
  const {
    workflow: { db_flow_id, db_flow_version_id, selected_node_data },
    explorer: { insertVariable },
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
          account: selectedAccount?.account_id,
        });
        return;
      }

      console.log("[RESULTS EXPLORER] Fetching variables for:", {
        account_id: selectedAccount.account_id,
        flow_id: db_flow_id,
        version_id: db_flow_version_id,
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
    <div className="h-full overflow-y-auto">
      <ScrollArea>
        <div className="grid w-full items-start gap-6 p-2">
          {selected_node_data &&
            selected_node_data.type !== ActionType.Trigger && (
              <div className="rounded-lg border p-4">
                <Header />
                {loading && <div>Loading...</div>}
                {results.length === 0 && !loading && (
                  <div className="text-muted-foreground">
                    Run Workflow Test Access Results
                  </div>
                )}
                {results.map(
                  (task: TaskRow) =>
                    task.type === "action" && (
                      <div key={task.task_id} className="flex flex-col">
                        <div className="flex-1">{task.action_label}</div>
                        <JsonExplorer
                          data={task.result}
                          onSelect={(v) => {
                            console.log(v);
                            insertVariable(
                              `{{${v}}}`, // Or whatever field name you're targeting
                            );
                          }}
                        />
                      </div>
                    ),
                )}
              </div>
            )}
        </div>
      </ScrollArea>
    </div>
  );
}
