import {
  BigFlow,
  flowJsonFromBigFlow,
  FlowTemplate,
  Profile,
} from "@anything/utils";

import { CommonProps } from "../types/commonTypes";
import { AvatarAndUsername } from "./avatarAndUsername";
import { BaseNodeWeb } from "./baseNodeWeb";
import { ProfileLinks } from "./profileLinks";
import { Tags } from "./tags";

interface TemplateViewProps extends CommonProps {
  template: BigFlow;
  profile: Profile | undefined;
}

const TemplateView = ({
  template,
  profile,
  Link,
  Avatar,
}: TemplateViewProps) => {
  let flow = flowJsonFromBigFlow(template) as FlowTemplate;

  return (
    <>
      <div className="min-h-16 mb-6 text-3xl font-semibold md:text-5xl">
        {template.flow_template_name}
      </div>
      <div className="flex flex-row justify-between">
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
        <div>
          {/* //TODO: deeplink or no */}
          <button>
            {/* <Deeplink href="anything://templateid">Open in App </Deeplink> */}
            <a href={`anything://templateid`}>Open in App</a>
          </button>
        </div>
      </div>
      <div className="mb-2 mt-8 font-semibold">About this template</div>
      <div className="">{template.flow_template_description}</div>

      <div className="mb-2 mt-8 font-semibold">Trigger</div>
      <div>
        <BaseNodeWeb node={flow.trigger} />
      </div>
      <div className="mb-2 mt-8 font-semibold">Actions</div>
      <div>
        {flow.actions.map((action) => {
          return <BaseNodeWeb node={action} key={action.node_label} />;
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
};

export default TemplateView;
