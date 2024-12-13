import { useAnything } from "@/context/AnythingContext";
import { ActionType } from "@/types/workflows";
import { useEffect, useState } from "react";
import api from "@repo/anything-api";
import { useAccounts } from "@/context/AccountsContext";
import { JsonExplorer } from "./json-explorer";
import { ScrollArea } from "@repo/ui/components/ui/scroll-area";
import { Button } from "@repo/ui/components/ui/button";
import { Send, XIcon } from "lucide-react";
import { createClient } from "@/lib/supabase/client";

export function VariablesExplorer(): JSX.Element {
  const {
    workflow: {
      db_flow_id,
      db_flow_version_id,
      selected_node_data,
      setShowExplorer,
    },
    explorer: { insertVariable },
  } = useAnything();

  const [variables, setVariables] = useState<any>({});
  const [renderedVariables, setRenderedVariables] = useState<any>({});
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
        console.log("[VARIABLES EXPLORER] Missing required IDs:", {
          db_flow_id,
          db_flow_version_id,
          account: selectedAccount?.account_id,
        });
        return;
      }

      console.log("[VARIABLES EXPLORER] Fetching variables for:", {
        account_id: selectedAccount.account_id,
        flow_id: db_flow_id,
        version_id: db_flow_version_id,
      });

      const result = await api.variables.getWorkflowVersionVariables(
        await createClient(),
        selectedAccount.account_id,
        db_flow_id,
        db_flow_version_id,
        selected_node_data?.action_id,
      );

      if (result) {
        console.log(
          "[VARIABLES EXPLORER] Successfully fetched results:",
          result,
        );
        setRenderedVariables(result.rendered_variables);
        setVariables(result.variables);
      } else {
        console.error("[VARIABLES EXPLORER] No results found");
      }
    } catch (error) {
      console.error("[VARIABLES EXPLORER] Error fetching results:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    console.log("[VARIABLES EXPLORER] Initial fetch triggered");
    fetchResults();
  }, [selected_node_data?.action_id]);

  return (
    <div className="h-full w-full flex flex-col">
      <div className="pt-2 pl-2">
        <Button
          onClick={() => setShowExplorer(false)}
          variant="outline"
          size="icon"
          aria-label="Close"
        >
          <XIcon className="size-5 fill-foreground" />
        </Button>
      </div>
      <ScrollArea className="flex-1">
        <div className="px-2">
          {selected_node_data &&
            selected_node_data.type !== ActionType.Trigger && (
              <div className="w-full">
                {/* <Header /> */}
                {loading && <div>Loading...</div>}
                {!variables && !loading && (
                  <div className="text-muted-foreground">
                    Run Workflow Test Access Variables
                  </div>
                )}
                {/* TODO: probably actually loop through the keys of variables and have an explorer for each */}
                {renderedVariables && (
                  <div className="h-auto w-full my-2 flex flex-col bg-white bg-opacity-5 overflow-hidden border rounded-md">
                    <div className="p-3">
                      <div className="flex-1 font-bold mb-2">Variables</div>
                      <div className="w-full rounded-lg p-2.5 bg-[whitesmoke]">
                        <JsonExplorer
                          parentPath={"variables."}
                          data={renderedVariables}
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
                )}
              </div>
            )}
        </div>
      </ScrollArea>
    </div>
  );
}
