"use client";

import { TemplateGrid } from "@repo/ui/components/templateGrid";
import Link from "next/link";
import { useEffect, useState } from "react";
import api from "@repo/anything-api";

import { Avatar } from "@/components/avatar";

export default function TemplatePage() {
  const [actionTemplates, setActionTemplates] = useState([]);
  const [error, setError] = useState(null);

  useEffect(() => {
    async function fetchTemplates() {
      try {
        const templates =
          await api.action_templates.getActionTemplatesForMarketplace();
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
      <div className="mt-16 flex flex-col items-center gap-4">
        <h1 className="h2 md:display  w-full px-4 text-center md:w-[805px] md:px-0">
          Anything Templates
        </h1>
        <p className="body-xl text-slate-11 w-full px-4 text-center md:w-[572px] md:px-0">
          Automate anything with easy to customize templates
        </p>
      </div>

      {/* Grid */}
      <div className="my-16 flex flex-col items-center">
        {actionTemplates.length > 0 && (
          <TemplateGrid
            AvatarComponent={Avatar}
            LinkComponent={Link}
            templates={actionTemplates}
          />
        )}
        {/* {error && <p>Error loading templates: {error.message}</p>} */}
      </div>
    </>
  );
}
