import { useAnything } from "@/context/AnythingContext";
import { ActionType } from "@/types/workflows";
import { useEffect, useState } from "react";
import api, { TaskRow } from "@repo/anything-api";
import { useAccounts } from "@/context/AccountsContext";
import { JsonExplorer } from "./json-explorer";
import { ScrollArea } from "@repo/ui/components/ui/scroll-area";

export function ResultsExplorer(): JSX.Element {
  const {
    workflow: {
      db_flow_id,
      db_flow_version_id,
      selected_node_data,
      setShowExplorer,
    },
    explorer: { insertVariable },
  } = useAnything();

  const [results, setResults] = useState<TaskRow[]>([]);
  const [loading, setLoading] = useState(false);
  const { selectedAccount } = useAccounts();
  const fetchResults = async () => {
    try {
      setLoading(true);
      if (
        !db_flow_id ||
        !db_flow_version_id ||
        !selectedAccount ||
        !selected_node_data ||
        !selected_node_data.action_id
      ) {
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
        selected_node_data?.action_id,
      );

      if (result) {
        console.log("[RESULTS EXPLORER] Successfully fetched results:", result);
        setResults(result.tasks);
      } else {
        console.error("[RESULTS EXPLORER] No results found");
      }
    } catch (error) {
      console.error("[RESULTS EXPLORER] Error fetching results:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    console.log("[RESULTS EXPLORER] Initial fetch triggered");
    fetchResults();
  }, [selected_node_data]);

  return (
    <div className="h-full w-full flex flex-col">
      <div className="p-2">
        <button
          onClick={() => setShowExplorer(false)}
          className="text-muted-foreground hover:text-foreground"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
            className="h-4 w-4"
          >
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>
      <ScrollArea className="flex-1">
        <div className="p-2">
          {selected_node_data &&
            selected_node_data.type !== ActionType.Trigger && (
              <div className="w-full">
                {/* <Header /> */}
                {loading && <div>Loading...</div>}
                {results && results.length === 0 && !loading && (
                  <div className="text-muted-foreground">
                    Run Workflow Test Access Results
                  </div>
                )}
                {results &&
                  results.map((task: TaskRow) => (
                    <div
                      key={task.task_id}
                      // className="h-auto w-full my-2 flex flex-col bg-white bg-opacity-5 border rounded-md p-3 text-primary-content"
                      className="h-auto w-full my-2 flex flex-col bg-white bg-opacity-5 overflow-hidden border rounded-md" // Added overflow-hidden
                    >
                      <div className="p-3">
                        <div className="flex-1 font-bold mb-2">
                          {task.action_label}
                        </div>
                        <div className="w-full rounded-lg p-2.5 bg-[whitesmoke]">
                            <JsonExplorer
                              parentPath={"actions." + task.action_id + "."}
                              data={task.result}
                              onSelect={(v) => {
                                console.log(v);
                                insertVariable(
                                  `{{${v}}}`, // Or whatever field name you're targeting
                                );
                              }}
                            />
                        </div>
                      </div>
                    </div>
                  ))}
              </div>
            )}
        </div>
      </ScrollArea>
    </div>
  );
}
