import { TemplateGrid } from "@repo/ui/components/templateGrid";
import Link from "next/link";
import api, { DBFlowTemplate } from "@repo/anything-api";

import { Avatar } from "@/components/avatar";

export default async function WorkflowTemplates() {
  let workflowTemplates: DBFlowTemplate[] = [];
  let error = null;

  try {
    const templates =
      await api.marketplace.getWorkflowTemplatesForMarketplace();
    if (templates && templates.length > 0) {
      workflowTemplates = templates;
    } else {
      console.log("No templates found");
    }
  } catch (err) {
    const errorMessage =
      err instanceof Error ? err.message : "An unknown error occurred";
    error = errorMessage;
    console.error("Error fetching action templates:", errorMessage);
  }

  return (
    <>
      {/* Hero Copy */}
      <div className="mt-24 flex flex-col items-center gap-8">
        <h1 className="text-5xl md:text-7xl font-bold text-center max-w-4xl leading-tight">
          Jumpstart Your Automation Journey with Our Templates
        </h1>
        <p className="text-2xl md:text-3xl text-slate-11 max-w-3xl text-center leading-relaxed">
          Discover pre-built workflows to help you get started quickly and
          easily
        </p>
      </div>

      {/* Grid */}
      <div className="my-16 flex flex-col items-center">
        {workflowTemplates.length > 0 && (
          <TemplateGrid
            AvatarComponent={Avatar}
            LinkComponent={Link}
            templates={workflowTemplates}
          />
        )}
        {error && <p>Error loading templates: {error}</p>}
      </div>
    </>
  );
}
