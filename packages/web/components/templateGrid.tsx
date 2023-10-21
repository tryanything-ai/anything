import { BigFlow } from "@/lib/fetchSupabase";
import { TemplateCard } from "./templateCard";
import { flowJsonFromBigFLow } from "@/utils/frontEndUtils";

export const TemplateGrid = ({ templates, profile = true }: { templates: BigFlow, profile?: boolean }) => {
  return (
    <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 3xl:grid-cols-4 gap-6 mx-auto max-w-7xl">
      {templates.map((template, index) => {

        let flow_json = flowJsonFromBigFLow(template);
       
        return (
          <TemplateCard
            key={index}
            profile={profile}
            tags={template.tags}
            avatar_url={template?.profiles?.avatar_url || ""}
            username={template?.profiles?.username || ""}
            profile_name={template?.profiles?.full_name || ""}
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
