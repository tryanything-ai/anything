import { TemplateCard } from "@/components/templateCard";
import { Database } from "@/types/supabase.types";

type Flow = Database["public"]["Tables"]["flow_templates"]["Row"];

export default function TemplatePage() {
  return (
    <>
      {/* Hero Copy */}
      <div className="mt-16 flex flex-col items-center gap-4">
        <h1 className="md:h1 h2 w-full px-4 text-center md:w-[805px] md:px-0">
          Anything Templates
        </h1>
        <p className="body-xl w-full px-4 text-center text-slate-11 md:w-[572px] md:px-0">
          Automate anything with ready to use templates
        </p>
      </div>

      {/* Pricing */}
      <div className="my-16 flex flex-col items-center">
        <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 3xl:grid-cols-4 gap-6 mx-auto max-w-7xl">
          {/* {mockRows.map((template, index) => (
            <TemplateCard key={index} template={template} />
          ))} */}
        </div>
      </div>
    </>
  );
}
