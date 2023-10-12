import { BigFlow, FlowTemplateVersion } from "@/lib/fetchSupabase";
import { CardProps, TemplateCard } from "./templateCard";

export const TemplateGrid = ({ templates }: { templates: BigFlow }) => {
  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 3xl:grid-cols-4 gap-6 mx-auto max-w-7xl">
      {templates.map((template, index) => {
        // TODO: this whole thing is kinda garbage and related to typescript problems with supabase queryes that are nested
        let flow_json = {};
        if (
          template.flow_template_versions &&
          Array.isArray(template.flow_template_versions)
        ) {
          flow_json = template.flow_template_versions[0].flow_template_json;
        } else {
          return null;
        }

        return (
          <TemplateCard
            key={index}
            tags={template.tags}
            description={
              template.flow_template_description
                ? template.flow_template_description
                : ""
            }
            flow_template_json={flow_json}
            slug={template.slug}
            flow_name={template.flow_template_name}
          />
        );
      })}
    </div>
  );
};
