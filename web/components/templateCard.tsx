import Link from "next/link";
import { Database } from "@/types/supabase.types";

type Flow = Database["public"]["Tables"]["flow_templates"]["Row"];

export async function TemplateCard({ template }: { template: Flow }) {
  return (
    <Link
      href={template.publisher_id}
      className="bg-base-300 rounded-lg overflow-hidden transition-all duration-200 ease-in-out transform hover:scale-105"
    >
      {/* <img 
            src={template.image}
            alt={template.name}
            className="w-full h-48 object-cover"
        /> */}
      <div className="p-6">
        {/* <h2 className="text-lg font-semibold text-gray-700">{template.f}</h2> */}
        {/* <p className="text-gray-500">{template}</p> */}
      </div>
    </Link>
  );
}
