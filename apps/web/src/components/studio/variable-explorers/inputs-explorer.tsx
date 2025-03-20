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

// Base component that just handles the variables display
export function BaseInputsExplorer(): JSX.Element {
  const {
    workflow: { db_flow_id, db_flow_version_id, selected_node_data },
    explorer: { insertVariable },
  } = useAnything();

  const [inputs, setInputs] = useState<any>({});
  // const [renderedInputs, setRenderedInputs] = useState<any>({});
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

      const result = await api.variables.getWorkflowVersionPluginInputs(
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

        if (result.rendered_inputs) {
          setInputs(result.rendered_inputs);
        } else {
          setInputs(result.inputs);
        }
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
    <div className="flex flex-col h-full w-full overflow-y-auto">
      <ScrollArea className="h-full">
        {selected_node_data &&
          selected_node_data.type !== ActionType.Trigger && (
            <div className="w-full">
              {loading && <div>Loading...</div>}
              {!inputs && !loading && (
                <div className="text-muted-foreground">
                  Run Workflow Test To Access Variables
                </div>
              )}
              {inputs && (
                <div className="h-auto w-full my-2 flex flex-col bg-white bg-opacity-5 overflow-hidden border rounded-md">
                  <div className="p-3">
                    <div className="flex-1 font-bold mb-2">Inputs</div>
                    <div className="w-full rounded-lg p-2.5 bg-[whitesmoke]">
                      <JsonExplorer
                        parentPath={"inputs."}
                        data={inputs}
                        onSelect={(v) => {
                          console.log(v);
                          insertVariable(`{{${v}}}`);
                        }}
                      />
                    </div>
                  </div>
                </div>
              )}
            </div>
          )}
      </ScrollArea>
    </div>
  );
}

// Wrapper component with layout controls
export function InputsExplorer(): JSX.Element {
  const {
    workflow: { setShowExplorer },
  } = useAnything();

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
      {/* <ScrollArea className="flex-1"> */}
      <div className="flex-1 min-h-0 h-full px-2">
        <BaseInputsExplorer />
      </div>
      {/* </ScrollArea> */}
    </div>
  );
}
