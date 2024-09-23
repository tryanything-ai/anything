import { flowJsonFromBigFlow } from "../helpers/helpers";
import type { ComponentType } from "react";
import type { CommonProps } from "./commonTypes";
import { AvatarAndUsername } from "./avatarAndUsername";
import { BaseNodeWeb } from "./baseNodeWeb";
import { ProfileLinks } from "./profileLinks";
import { Tags } from "./tags";
import api, { DBFlowTemplate } from "@repo/anything-api";

import { ActionNode } from "./action-grid";

interface TemplateViewProps extends CommonProps {
  template: DBFlowTemplate;
  profile: any | undefined;
  ActionComponent: ComponentType<any>;
}

export const TemplateView = ({
  template,
  profile,
  Link,
  Avatar,
  ActionComponent,
}: TemplateViewProps) => {
  // let flow = flowJsonFromBigFlow(template);
  // console.log("Flow JSON in TemplateView:", flow);

  const getFlowDetails = (template: DBFlowTemplate) => {
    const latestVersion = template.flow_template_versions[0];
    if (!latestVersion || !latestVersion.flow_definition) {
      return { trigger: null, actions: [] };
    }

    const { actions } = latestVersion.flow_definition;
    const trigger = actions.find((action) => action.type === "trigger");
    const nonTriggerActions = actions.filter(
      (action) => action.type !== "trigger",
    );

    return { trigger, actions: nonTriggerActions };
  };

  const { trigger, actions } = getFlowDetails(template);

  console.log("Trigger:", trigger);
  console.log("Actions:", actions);

  return (
    <>
      <div className="min-h-16 mb-16 text-3xl w-full  font-semibold md:text-5xl">
        {template.flow_template_name}
      </div>
      <div className="flex flex-col md:flex-row gap-4 justify-between ">
        {/* Left */}
        <div>
          <AvatarAndUsername
            AvatarComponent={() =>
              Avatar({
                avatar_url: profile?.avatar_url ? profile.avatar_url : "",
                profile_name: profile?.full_name ? profile.full_name : "",
              })
            }
            profile_name={profile?.full_name ? profile.full_name : ""}
            username={profile?.username ? profile.username : ""}
            Link={Link}
          />
        </div>
        {/* Right */}
        {/* <div> */}
        {/* //TODO: deeplink or no */}
        <ActionComponent profile={profile} template={template} />
        {/* <button>
             <Deeplink href="anything://templateid">Open in App </Deeplink> 
            <a href={`anything://templates/${template.slug}`}>Open in App</a>
          </button> */}
        {/* </div> */}
      </div>
      <div className="mb-2 mt-8 font-semibold">About this template</div>
      <div className="">{template.flow_template_description}</div>

      <div className="mb-2 mt-8 font-semibold">Trigger</div>
      <div>
        {trigger && (
          <ActionNode
            id={trigger.action_id || ""}
            name={trigger.label || ""}
            description={trigger.description || ""}
            icon={trigger.icon || ""}
          />
        )}
        {/* <BaseNodeWeb node={flow.trigger} /> */}
      </div>
      <div className="mb-2 mt-8 font-semibold">Actions</div>
      <div>
        {actions.map((action: any) => {
          return (
            <ActionNode
              id={action.action_id}
              key={action.action_id}
              name={action.label || ""}
              description={action.description || ""}
              icon={action.icon || ""}
            />
          );
        })}
      </div>
      <div className="mb-2 mt-8 font-semibold">Tags</div>
      <Tags tags={template.tags} />
      {profile ? (
        <>
          <div className="mb-2 mt-8 font-semibold">About the creator</div>
          <ProfileLinks profile={profile} Link={Link as any} />
        </>
      ) : null}
    </>
  );
};
