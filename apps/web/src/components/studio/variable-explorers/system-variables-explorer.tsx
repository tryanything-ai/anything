import { useAnything } from "@/context/AnythingContext";
import { useEffect, useState } from "react";
import api from "@repo/anything-api";
import { useAccounts } from "@/context/AccountsContext";
import { JsonExplorer } from "./json-explorer";
import { createClient } from "@/lib/supabase/client";
export function SystemVariablesExplorer(): JSX.Element {
  const {
    workflow: { selected_node_data },
    explorer: { insertVariable },
  } = useAnything();

  const [systemVariables, setSystemVariables] = useState<Record<string, any>>(
    {},
  );
  const [loading, setLoading] = useState(false);
  const { selectedAccount } = useAccounts();

  const fetchSystemVariables = async () => {
    try {
      setLoading(true);
      if (!selectedAccount) {
        console.error("No account selected");
        return;
      }
      const response = await api.variables.getSystemVariables(
        await createClient(),
        selectedAccount.account_id,
      );
      if (!response) {
        setSystemVariables({});
        return;
      }

      setSystemVariables(response);
    } catch (error) {
      console.error("Error fetching system variables:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    console.log("[SYSTEM VARIABLES EXPLORER] Initial fetch triggered");
    fetchSystemVariables();
  }, [selected_node_data?.action_id]);

  return (
    <div className="flex flex-col w-full">
      {selected_node_data && (
        <div className="w-full">
          {loading && <div>Loading...</div>}
          {Object.keys(systemVariables).length === 0 && !loading && (
            <div className="text-muted-foreground p-3">
              No system variables found.
            </div>
          )}
          {Object.keys(systemVariables).length > 0 && (
            <div className="h-auto w-full flex flex-col bg-white bg-opacity-5 overflow-hidden border rounded-md">
              <div className="p-3">
                <div className="flex-1 font-bold mb-2">System Variables</div>
                <div className="w-full rounded-lg p-2.5 bg-[whitesmoke]">
                  <JsonExplorer
                    parentPath="system."
                    data={systemVariables}
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
    </div>
  );
}
