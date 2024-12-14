"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { useParams } from "next/navigation";
import api from "@repo/anything-api";
import { Loader2 } from "lucide-react";
import { useAnything } from "@/context/AnythingContext";
import { createClient } from "@/lib/supabase/client";
export default function TemplateLoadingPage() {
  const router = useRouter();
  const params = useParams();

  const {
    accounts: { selectedAccount },
  } = useAnything();

  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const cloneTemplate = async () => {
    try {
      if (!selectedAccount) {
        throw new Error("No account selected");
      }

      console.log("Cloning template...");

      const templateId = params.templateId as string;

      const clonedTemplateResponse: {
        flow_id: string;
        flow_version_id: string;
      } = await api.marketplace.cloneWorkflowTemplate(
        await createClient(),
        selectedAccount.account_id,
        templateId,
      );

      if (
        clonedTemplateResponse.flow_id &&
        clonedTemplateResponse.flow_version_id
      ) {
        router.push(
          `/workflows/${clonedTemplateResponse.flow_id}/${clonedTemplateResponse.flow_version_id}/editor`,
        );
      } else {
        throw new Error("Failed to clone template");
      }
    } catch (err) {
      console.error("Error cloning template:", err);
      setError("Failed to load template. Please try again.");
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    cloneTemplate();
  }, [params.templateId, router]);

  if (error) {
    return (
      <div className="flex flex-col items-center justify-center h-screen">
        <p className="text-red-500 mb-4">{error}</p>
        {/* <button
          onClick={() => router.push("/templates")}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Back to Templates
        </button> */}
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen">
      {isLoading && (
        <>
          <Loader2 className="h-8 w-8 animate-spin text-blue-500 mb-4" />
          <p className="text-lg">Loading template...</p>
        </>
      )}
    </div>
  );
}
