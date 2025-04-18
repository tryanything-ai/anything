// import type { BigFlow} from "utils";
import { flowJsonFromBigFlow } from "../helpers/helpers";
import type { ComponentType, JSX } from "react";
import TemplateCard from "./templateCard";
import { DBFlowTemplate } from "@repo/anything-api";

export const TemplateGrid = ({
  templates,
  AvatarComponent,
  LinkComponent,
  profile = true,
}: {
  templates: DBFlowTemplate[];
  AvatarComponent: (props: {
    avatar_url: string;
    profile_name: string;
  }) => JSX.Element;
  LinkComponent: ComponentType<any>;
  profile?: boolean;
}) => {
  return (
    <div className="3xl:grid-cols-4 mx-auto grid max-w-7xl grid-cols-1 gap-6 lg:grid-cols-2 xl:grid-cols-3">
      {templates.map((template: any) => {
        // const flowJson = flowJsonFromBigFlow(template);
        // console.log("Flow JSON:", flowJson);

        return (
          <TemplateCard
            AvatarComponent={() =>
              AvatarComponent({
                avatar_url: template?.profiles?.avatar_url || "",
                profile_name: template?.profiles?.full_name || "",
              })
            }
            Link={LinkComponent}
            description={
              template.flow_template_description
                ? template.flow_template_description
                : ""
            }
            flowName={template.flow_template_name}
            template={template}
            key={template.flow_template_id}
            profile={profile}
            profileName={template?.profiles?.full_name || ""}
            slug={template.slug}
            username={template?.profiles?.username || ""}
          />
        );
      })}
    </div>
  );
};
