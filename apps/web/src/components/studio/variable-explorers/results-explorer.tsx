import { useAnything } from "@/context/AnythingContext";
import { ActionType } from "@/types/workflows";
import { useEffect, useState } from "react";
import api, { TaskRow } from "@repo/anything-api";
import { useAccounts } from "@/context/AccountsContext";
import { JsonExplorer } from "./json-explorer";
import { Button } from "@repo/ui/components/ui/button";
import { ChevronDown, ChevronRight } from "lucide-react";
import { SvgRenderer } from "../nodes/node-icon";

export function ResultsExplorer(): JSX.Element {
  const {
    workflow: {
      db_flow_id,
      db_flow_version_id,
      selected_node_data,
      getActionIcon,
    },
    explorer: { insertVariable },
  } = useAnything();

  const [results, setResults] = useState<TaskRow[]>([]);
  const [loading, setLoading] = useState(false);
  const [expandedTasks, setExpandedTasks] = useState<Record<string, boolean>>(
    {},
  );
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

      const result = await api.variables.getWorkflowVersionResults(
        selectedAccount.account_id,
        db_flow_id,
        db_flow_version_id,
        selected_node_data?.action_id,
      );

      if (result) {
        console.log("[RESULTS EXPLORER] Successfully fetched results:", result);
        setResults(result.tasks);
        // Initialize all tasks as collapsed
        const initialExpandState = result.tasks.reduce(
          (acc: any, task: any) => {
            acc[task.task_id] = false;
            return acc;
          },
          {} as Record<string, boolean>,
        );
        setExpandedTasks(initialExpandState);
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
  }, [selected_node_data?.action_id]);

  const toggleTaskExpansion = (taskId: string) => {
    setExpandedTasks((prev) => ({
      ...prev,
      [taskId]: !prev[taskId],
    }));
  };

  return (
    <div className="flex flex-col w-full">
      {selected_node_data && selected_node_data.type !== ActionType.Trigger && (
        <div className="w-full">
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
                className="h-auto w-full my-2 flex flex-col bg-white bg-opacity-5 overflow-hidden border rounded-md"
              >
                <div className="p-3">
                  <div 
                    className="flex items-center mb-2 cursor-pointer" 
                    onClick={() => toggleTaskExpansion(task.task_id)}
                  >
                    <div className="p-0 h-6 w-6 mr-1 flex items-center justify-center">
                      {expandedTasks[task.task_id] ? (
                        <ChevronDown className="h-4 w-4" />
                      ) : (
                        <ChevronRight className="h-4 w-4" />
                      )}
                    </div>
                    <div className="flex-1 flex flex-row font-bold">
                      <div className="h-6 w-6 mr-2">
                        <SvgRenderer
                          className={`h-5 w-5`}
                          icon={getActionIcon(task.action_id)}
                        />
                      </div>
                      {task.action_label}
                    </div>
                  </div>
                  {expandedTasks[task.task_id] && (
                    <div className="w-full rounded-lg p-2.5 bg-[whitesmoke]">
                      <JsonExplorer
                        parentPath={
                          "actions." + task.action_id + "." + "result."
                        }
                        data={task.result}
                        onSelect={(v) => {
                          console.log(v);
                          insertVariable(`{{${v}}}`);
                        }}
                      />
                    </div>
                  )}
                </div>
              </div>
            ))}
        </div>
      )}
    </div>
  );
}
