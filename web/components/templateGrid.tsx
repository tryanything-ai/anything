import { FlowTemplateVersion } from "@/lib/fetchSupabase";
import { TemplateCard } from "./templateCard";

export const TemplateGrid = ({
  templates,
}: {
  templates: FlowTemplateVersion[];
}) => {
  return (
    <div className="my-16 flex flex-col items-center">
      <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 3xl:grid-cols-4 gap-6 mx-auto max-w-7xl">
        {templates.map((template, index) => {
          return (
            <TemplateCard
              key={index}
              flow_template_json={template.flow_template_json}
              slug={template.slug}
              flow_name={template.flow_template_version_name}
            />
          );
        })}
      </div>
    </div>
  );
};
