import { BigFlow,flowJsonFromBigFLow } from "@anything/utils";
import React from "react"

import TemplateCard from "./templateCard";

const TemplateGrid = ({
  templates,
  Link,
  AvatarComponent,
  profile = true,
}: {
  templates: BigFlow;
  Link: React.ComponentType<any>;
  AvatarComponent: React.ComponentType;
  profile?: boolean;
}) => {
  return (
    <div className="3xl:grid-cols-4 mx-auto grid max-w-7xl grid-cols-1 gap-6 lg:grid-cols-2 xl:grid-cols-3">
      {templates.map((template, index: number) => {
        let flow_json = flowJsonFromBigFLow(template);
        // get profile name
        //get avatar url from rpfoile

        return (
          <TemplateCard
            AvatarComponent={() => (
            <AvatarComponent />
            )}
            Link={Link}
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

export default TemplateGrid;