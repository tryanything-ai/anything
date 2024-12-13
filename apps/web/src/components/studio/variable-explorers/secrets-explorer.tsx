import { useAnything } from "@/context/AnythingContext";
import { ActionType } from "@/types/workflows";
import { useEffect, useState } from "react";
import api from "@repo/anything-api";
import { useAccounts } from "@/context/AccountsContext";
import { Button } from "@repo/ui/components/ui/button";
import { Eye, EyeOff } from "lucide-react";
import { createClient } from "@/lib/supabase/client";

interface Secret {
  secret_id: string;
  secret_name: string;
  secret_value: string;
}

function SecretRow({
  secret,
  onInsert,
}: {
  secret: Secret;
  onInsert: (value: string) => void;
}) {
  const [isVisible, setIsVisible] = useState(false);

  return (
    <div key={secret.secret_id} className="flex items-center gap-2">
      <Button
        variant="ghost"
        className="p-1 m-1 h-auto bg-blue-500 text-blue-100 hover:bg-blue-600 hover:text-blue-50 font-medium"
        onClick={() => onInsert(`{{secrets.${secret.secret_name}}}`)}
      >
        {secret.secret_name}
      </Button>
      <Button
        variant="ghost"
        size="sm"
        className=""
        onClick={() => setIsVisible(!isVisible)}
      >
        {isVisible ? (
          <EyeOff className="h-4 w-4" />
        ) : (
          <Eye className="h-4 w-4" />
        )}
      </Button>

      {isVisible ? (
        <span className="text-gray-400 text-sm">{secret.secret_value}</span>
      ) : (
        <span className="text-gray-400 text-lg tracking-widest">••••••••</span>
      )}
    </div>
  );
}

export function SecretsExplorer(): JSX.Element {
  const {
    workflow: { selected_node_data },
    explorer: { insertVariable },
  } = useAnything();

  const [secrets, setSecrets] = useState<Secret[]>([]);
  const [loading, setLoading] = useState(false);
  const { selectedAccount } = useAccounts();

  const fetchSecrets = async () => {
    try {
      setLoading(true);
      if (!selectedAccount) {
        console.error("No account selected");
        return;
      }
      const response = await api.secrets.getSecrets(
        await createClient(),
        selectedAccount.account_id,
      );
      if (response.length === 0) {
        setSecrets([]);
        return;
      }

      setSecrets(response);
    } catch (error) {
      console.error("Error fetching secrets:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    console.log("[SECRETS EXPLORER] Initial fetch triggered");
    fetchSecrets();
  }, [selected_node_data?.action_id]);

  return (
    <div className="flex flex-col w-full">
      {selected_node_data && (
        <div className="w-full">
          {loading && <div>Loading...</div>}
          {secrets && secrets.length === 0 && !loading && (
            <div className="text-muted-foreground p-3">
              No secrets found. Add secrets in the Connections page.
            </div>
          )}
          {secrets && secrets.length > 0 && (
            <div className="h-auto w-full flex flex-col bg-white bg-opacity-5 overflow-hidden border rounded-md">
              <div className="p-3">
                <div className="flex-1 font-bold mb-2">Secrets</div>
                <div className="w-full rounded-lg p-2.5 bg-[whitesmoke]">
                  {secrets.map((secret) => (
                    <SecretRow
                      key={secret.secret_id}
                      secret={secret}
                      onInsert={insertVariable}
                    />
                  ))}
                </div>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
