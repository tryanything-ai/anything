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
      <div className="flex items-center justify-end p-2">
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
      <ScrollArea>
        <div className="grid w-full items-start gap-6 p-2">
          {selected_node_data &&
            selected_node_data.type !== ActionType.Trigger && (
              <div className="rounded-lg border p-4">
                <Header />
                {loading && <div>Loading...</div>}
                {results && results.length === 0 && !loading && (
                  <div className="text-muted-foreground">
                    Run Workflow Test Access Results
                  </div>
                )}
                {results &&
                  results.map((task: TaskRow) => (
                    <div key={task.task_id} className="flex flex-col">
                      <div className="flex-1">{task.action_label}</div>
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
                  ))}
              </div>
            )}
        </div>
      </ScrollArea>
    </div>
  );
}
