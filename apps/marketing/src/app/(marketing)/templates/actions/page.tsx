import { ActionTemplateGrid } from "@repo/ui/components/action-grid";
import api from "@repo/anything-api";

export default async function TemplatePage() {
  let actionTemplates = [];
  let error = null;

  try {
    const templates = await api.marketplace.getActionTemplatesForMarketplace();
    if (templates && templates.length > 0) {
      actionTemplates = templates;
    } else {
      console.log("No templates found");
    }
  } catch (err) {
    const errorMessage =
      err instanceof Error ? err.message : "An unknown error occurred";
    console.error("Error fetching action templates:", errorMessage);
    error = errorMessage;
  }

  return (
    <>
      {/* Hero Copy */}
      <div className="mt-24 flex flex-col items-center gap-8">
        <h1 className="text-5xl md:text-7xl font-bold text-center max-w-4xl leading-tight">
          Unleash the Power of Automation by Integrating your existing tools
        </h1>
        {/* <p className="text-2xl md:text-3xl text-slate-11 max-w-3xl text-center leading-relaxed">
          Transform Your Workflow: Discover, Customize, and Automate with Our
          Pre Built Integrations and Action Templates
        </p> */}
      </div>

      {/* Grid */}
      <div className="my-24 flex flex-col items-center">
        {actionTemplates.length > 0 && (
          <ActionTemplateGrid actionTemplates={actionTemplates} />
        )}
        {error && <p>Error loading templates: {error}</p>}
      </div>
    </>
  );
}
