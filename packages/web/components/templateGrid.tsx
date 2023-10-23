import { TemplateCard } from "@anything/ui";
import Image from "next/image";
import Link from "next/link";

import { BigFlow } from "@/lib/fetchSupabase";
import { flowJsonFromBigFLow } from "@/utils/frontEndUtils";

const Avatar = ({
  avatar_url,
  profile_name,
}: {
  avatar_url: string;
  profile_name: string;
}) => {
  return <Image width={100} height={100} src={avatar_url} alt={profile_name} />;
};

export const TemplateGrid = ({
  templates,
  profile = true,
}: {
  templates: BigFlow;
  profile?: boolean;
}) => {
  return (
    <div className="3xl:grid-cols-4 mx-auto grid max-w-7xl grid-cols-1 gap-6 lg:grid-cols-2 xl:grid-cols-3">
      {templates.map((template, index) => {
        let flow_json = flowJsonFromBigFLow(template);
        // get profile name
        //get avatar url from rpfoile

        return (
          <TemplateCard
            AvatarComponent={() => (
              <Avatar
                avatar_url={template?.profiles?.avatar_url || ""}
                profile_name={template?.profiles?.full_name || ""}
              />
            )}
            Link={Link}
            key={index}
            profile={profile}
            // tags={template.tags}
            // avatar_url={template?.profiles?.avatar_url || ""}
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
