import { BigFlow, flowJsonFromBigFlow, FlowTemplate, Profile } from "utils";
import type { ComponentType } from "react";
import type { CommonProps } from "./commonTypes";
import { AvatarAndUsername } from "./avatarAndUsername";
import { BaseNodeWeb } from "./baseNodeWeb";
import { ProfileLinks } from "./profileLinks";
import { Tags } from "./tags";

interface TemplateViewProps extends CommonProps {
  template: any;
  profile: Profile | undefined;
  ActionComponent: ComponentType<any>;
}

export function TemplateView({
  template,
  profile,
  Link,
  Avatar,
  ActionComponent,
}: TemplateViewProps) {
  let flow = flowJsonFromBigFlow(template);

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
        <ActionComponent />
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
        <BaseNodeWeb node={flow.trigger} />
      </div>
      <div className="mb-2 mt-8 font-semibold">Actions</div>
      <div>
        {flow.actions.map((action: any) => {
          return <BaseNodeWeb key={action.node_name} node={action} />;
        })}
      </div>
      <div className="mb-2 mt-8 font-semibold">Tags</div>
      <Tags tags={template.tags} />
      {profile ? (
        <>
          <div className="mb-2 mt-8 font-semibold">About the creator</div>
          <ProfileLinks profile={profile} Link={Link} />
        </>
      ) : null}
    </>
  );
}
