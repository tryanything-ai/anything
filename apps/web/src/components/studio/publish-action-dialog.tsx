import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@repo/ui/components/ui/alert-dialog";
import { Button } from "@repo/ui/components/ui/button";
import { useAnything } from "@/context/AnythingContext";
import { useState } from "react";
import api from "@repo/anything-api";
import { ActionType } from "@/types/workflows";

interface PublishActionDialogProps {
  show: boolean;
  onClose: () => void;
}

export default function PublishActionDialog({ show, onClose }: PublishActionDialogProps): JSX.Element {
  const {
    accounts: { selectedAccount },
    workflow: { selected_node_data },
  } = useAnything();

  const [checkedForSensitiveData, setCheckedForSensitiveData] = useState(false);
  const [publishToMarketplace, setPublishToMarketplace] = useState(false);
  const [publishAnonymously, setPublishAnonymously] = useState(false);
  const [publishToTeam, setPublishToTeam] = useState(true);
  const [success, setSuccess] = useState(false);
  const [marketplaceUrl, setMarketplaceUrl] = useState("");

  const handlePublish = async () => {
    try {
      console.log("Creating Action Template");
      if (!selectedAccount || !selected_node_data) return;
      if (!publishToTeam && !publishToMarketplace) {
        console.error("At least one publishing option must be selected");
        return;
      }

      if (!checkedForSensitiveData) {
        console.error("Must confirm checking for sensitive data");
        return;
      }

      // Update the action_id to match the plugin_id
      // Prevents node access slugs from propogating into marketplace from users local version that has some context
      // of the workflow it was included in like http_4 vs http
      if (selected_node_data.plugin_id) {
        selected_node_data.action_id = selected_node_data.plugin_id;
      } else {
        console.warn("No plugin_id found for the selected node");
      }

      console.log("Updated selected_node_data:", selected_node_data);

      let res = await api.action_templates.publishActionTemplate(
        selectedAccount.account_id,
        selected_node_data,
        publishToTeam,
        publishToMarketplace,
        publishAnonymously,
      );

      console.log("res from publish action template", res);

      if (res && res.marketplace_url) {
        setSuccess(true);
        setMarketplaceUrl(res.marketplace_url);
      }
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <>
      {selected_node_data && selected_node_data.type !== ActionType.Trigger && (
        <AlertDialog open={show} onOpenChange={onClose}>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>Publish Action Template</AlertDialogTitle>
              <AlertDialogDescription>
                Choose your publishing options:
              </AlertDialogDescription>
            </AlertDialogHeader>
            {!success ? (
              <>
                <div className="flex flex-col space-y-4">
                  <p className="text-yellow-600 font-semibold">
                    Before publishing make sure your template has no sensitive
                    data like API keys hard coded into the templates.
                  </p>
                  <label className="flex items-center space-x-2 mt-4">
                    <input
                      type="checkbox"
                      checked={checkedForSensitiveData}
                      onChange={(e) =>
                        setCheckedForSensitiveData(e.target.checked)
                      }
                    />
                    <span>I have checked my template for sensitive data</span>
                  </label>
                  <p className="text-sm text-gray-500 ml-6">
                    Confirm that you have reviewed your template and removed any
                    sensitive information such as API keys or personal data.
                  </p>
                  <label className="flex items-center space-x-2">
                    <input
                      type="checkbox"
                      checked={publishToTeam}
                      onChange={(e) => setPublishToTeam(e.target.checked)}
                    />
                    <span>Publish to Team</span>
                  </label>
                  <p className="text-sm text-gray-500 ml-6">
                    Make this action template available to all members of your
                    team as a reusable action.
                  </p>
                  <label className="flex items-center space-x-2 mt-4">
                    <input
                      type="checkbox"
                      checked={publishToMarketplace}
                      onChange={(e) =>
                        setPublishToMarketplace(e.target.checked)
                      }
                    />
                    <span>Publish to Marketplace</span>
                  </label>
                  <p className="text-sm text-gray-500 ml-6">
                    Share this action template with the entire Anything
                    community in the public marketplace.
                  </p>
                  <label className="flex items-center space-x-2 mt-4">
                    <input
                      type="checkbox"
                      checked={publishAnonymously}
                      onChange={(e) => setPublishAnonymously(e.target.checked)}
                    />
                    <span>Publish Anonymously</span>
                  </label>
                  <p className="text-sm text-gray-500 ml-6">
                    Hide your identity when publishing to the marketplace. Your
                    name won't be associated with this template.
                  </p>
                </div>
                <AlertDialogFooter>
                  <AlertDialogCancel onClick={onClose}>Cancel</AlertDialogCancel>
                  <AlertDialogAction
                    onClick={handlePublish}
                    disabled={
                      (!publishToTeam && !publishToMarketplace) ||
                      !checkedForSensitiveData
                    }
                  >
                    Publish
                  </AlertDialogAction>
                </AlertDialogFooter>
              </>
            ) : (
              <div className="flex flex-col items-center space-y-4">
                <p className="text-green-500 font-semibold">
                  Successfully published!
                </p>
                {marketplaceUrl && (
                  <a
                    href={marketplaceUrl}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-blue-500 hover:underline"
                  >
                    View in Marketplace
                  </a>
                )}
                <AlertDialogFooter>
                  <AlertDialogCancel onClick={() => {
                    setSuccess(false);
                    onClose();
                  }}>
                    Close
                  </AlertDialogCancel>
                </AlertDialogFooter>
              </div>
            )}
          </AlertDialogContent>
        </AlertDialog>
      )}
    </>
  );
}
