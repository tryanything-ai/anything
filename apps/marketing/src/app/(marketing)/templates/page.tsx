import { TemplateGrid } from "@repo/ui/components/templateGrid";
import { fetchTemplates } from "@/lib/supabase/fetchSupabase";
import Link from "next/link";
import { notFound } from "next/navigation";

import { Avatar } from "@/components/avatar";

import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Anything Templates",
  description: " Automate anything with easy to customize templates",
};

export default async function TemplatePage() {
  const templates: any = await fetchTemplates();

  if (!templates) {
    notFound();
  }

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
        <TemplateGrid
          AvatarComponent={Avatar}
          LinkComponent={Link}
          templates={templates}
        />
      </div>
    </>
  );
}
