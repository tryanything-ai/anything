import { BigFlow, flowJsonFromBigFlow } from "utils";
import { ComponentType, JSX } from "react";

import TemplateCard from "./templateCard";

export const TemplateGrid = ({
  templates,
  AvatarComponent,
  LinkComponent,
  profile = true,
}: {
  templates: BigFlow;
  AvatarComponent: (props: {
    avatar_url: string;
    profile_name: string;
  }) => JSX.Element;
  LinkComponent: ComponentType<any>;
  profile?: boolean;
}) => {
  return (
    <div className="3xl:grid-cols-4 mx-auto grid max-w-7xl grid-cols-1 gap-6 lg:grid-cols-2 xl:grid-cols-3">
      {templates.map((template: any, index: number) => {
        let flow_json = flowJsonFromBigFlow(template);

        return (
          <TemplateCard
            AvatarComponent={() =>
              AvatarComponent({
                avatar_url: template?.profiles?.avatar_url || "",
                profile_name: template?.profiles?.full_name || "",
              })
            }
            Link={LinkComponent}
            key={index}
            profile={profile}
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
