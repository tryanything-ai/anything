import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@repo/ui/components/ui/alert-dialog";
import { Button } from "@repo/ui/components/ui/button";
import { useAnything } from "@/context/AnythingContext";
import { useState } from "react";
import api from "@repo/anything-api";
import { ShareIcon } from "lucide-react";
import Link from "next/link";
import { useParams } from "next/navigation";
import { createClient } from "@/lib/supabase/client";

export function ShareDialog(): JSX.Element {
  const {
    accounts: { selectedAccount },
    workflow,
  } = useAnything();

  const [publishToMarketplace, setPublishToMarketplace] = useState(false);
  const [publishAnonymously, setPublishAnonymously] = useState(false);
  const [checkedForSensitiveData, setCheckedForSensitiveData] = useState(false);
  const [success, setSuccess] = useState(false);
  const [marketplaceUrl, setMarketplaceUrl] = useState("");
  const params = useParams<{ workflowVersionId: string; workflowId: string }>();

  const handlePublish = async () => {
    try {
      console.log("Publishing Flow Template");

      if (!selectedAccount || !workflow) return;

      if (!publishToMarketplace) {
        console.error("Publishing to marketplace must be selected");
        return;
      }

      if (!checkedForSensitiveData) {
        console.error("Must confirm checking for sensitive data");
        return;
      }

      if (!params.workflowId || !params.workflowVersionId) {
        console.error("Workflow ID or Workflow Version ID not found");
        return;
      }

      let res = await api.marketplace.publishFlowTemplateToMarketplace(
        await createClient(),
        selectedAccount.account_id,
        params.workflowId,
        params.workflowVersionId,
        publishAnonymously,
      );

      console.log("res from publish flow template", res);

      if (res && res.marketplace_url) {
        setSuccess(true);
        setMarketplaceUrl(res.marketplace_url);
      }
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <AlertDialog>
      <AlertDialogTrigger asChild>
        <Button variant="outline" size="sm" className="ml-auto gap-1.5 text-sm">
          <ShareIcon className="size-3.5" />
          Publish to Marketplace
        </Button>
      </AlertDialogTrigger>
      <AlertDialogContent className="sm:max-w-md">
        <AlertDialogHeader>
          <AlertDialogTitle>Share Workflow as Template</AlertDialogTitle>
        </AlertDialogHeader>
        {!success ? (
          <>
            <div className="flex flex-col space-y-4">
              <p>
                Other users will be able to use your template in their business
                from the{" "}
                <Link
                  href="https://tryanything.xyz/templates/workflows"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-blue-500 underline"
                >
                  Anything Marketplace
                </Link>
              </p>
              <p className="text-yellow-600 font-semibold">
                Before sharing to Marketplace make sure your template has no
                sensitive data like API keys hard coded into the templates.
              </p>
              <label className="flex items-center space-x-2 mt-4">
                <input
                  type="checkbox"
                  checked={checkedForSensitiveData}
                  onChange={(e) => setCheckedForSensitiveData(e.target.checked)}
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
                  checked={publishToMarketplace}
                  onChange={(e) => setPublishToMarketplace(e.target.checked)}
                />
                <span>Publish to Marketplace</span>
              </label>
              <p className="text-sm text-gray-500 ml-6">
                Share this flow template with the entire Anything community in
                the public marketplace.
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
                Hide your identity when publishing to the marketplace. Your name
                won't be associated with this template.
              </p>
            </div>
            <AlertDialogFooter>
              <AlertDialogCancel>Cancel</AlertDialogCancel>
              <AlertDialogAction
                onClick={handlePublish}
                disabled={!publishToMarketplace || !checkedForSensitiveData}
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
              <AlertDialogCancel onClick={() => setSuccess(false)}>
                Close
              </AlertDialogCancel>
            </AlertDialogFooter>
          </div>
        )}
      </AlertDialogContent>
    </AlertDialog>
  );
}
