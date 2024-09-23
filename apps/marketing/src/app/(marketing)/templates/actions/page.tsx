"use client";

import { TemplateGrid } from "@repo/ui/components/templateGrid";
import Link from "next/link";
import { useEffect, useState } from "react";
import api from "@repo/anything-api";
import { ActionTemplateGrid } from "@repo/ui/components/action-grid";
import { Avatar } from "@/components/avatar";

export default function TemplatePage() {
  const [actionTemplates, setActionTemplates] = useState([]);
  const [error, setError] = useState(null);

  useEffect(() => {
    async function fetchTemplates() {
      try {
        const templates =
          await api.marketplace.getActionTemplatesForMarketplace();
        if (templates && templates.length > 0) {
          setActionTemplates(templates);
        } else {
          console.log("No templates found");
        }
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : "An unknown error occurred";
        // setError(errorMessage as React.SetStateAction<null>);
        console.error("Error fetching action templates:", errorMessage);
      }
    }

    fetchTemplates();
  }, []);

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
        {/* {error && <p>Error loading templates: {error.message}</p>} */}
      </div>
    </>
  );
}
