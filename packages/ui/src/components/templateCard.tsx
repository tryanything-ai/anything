// import type { FlowTemplate, Json } from "utils";
import React from "react";
// import { VscArrowSmallRight } from "react-icons/vsc";
import { AvatarAndUsername } from "./avatarAndUsername";
import { BaseNodeIcon } from "./baseNodeIcons";

export interface TemplateCardProps {
  slug: string;
  description: string;
  profileName: string;
  profile: boolean;
  username: string;
  flowName: string;
  flowTemplateJson: any;
  Link: React.ComponentType<any>;
  AvatarComponent: React.ComponentType;
}

const TemplateCard = ({
  flowTemplateJson,
  // avatar_url,
  username,
  profileName,
  profile,
  // tags,
  slug,
  description,
  flowName,
  Link,
  AvatarComponent,
}: TemplateCardProps) => {
  const flowJson =
    typeof flowTemplateJson === "string"
      ? JSON.parse(flowTemplateJson)
      : flowTemplateJson;

  return (
    <Link
      data-ph-capture-attribute-flow-template-name={flowName}
      data-ph-capture-attribute-flow-template-slug={slug}
      href={`/templates/${slug}`}
      to={`/templates/${slug}`}
    >
      <div className="card card-compact bg-base-300 mx-1 max-w-md transform overflow-hidden shadow-xl transition-all duration-200 ease-in-out hover:scale-105 sm:w-96">
        <div className="card-body">
          <h2 className="card-title text-ellipsis text-2xl line-clamp-1">
            {flowName}
          </h2>

          {/* User */}
          {profile ? (
            <AvatarAndUsername
              AvatarComponent={AvatarComponent}
              Link={Link}
              link={false}
              profile_name={profileName}
              username={username}
            />
          ) : null}

          <p className="mt-1 line-clamp-2 overflow-hidden overflow-ellipsis h-12">
            {description}
          </p>
          <figure>
            <div className="mb-1 h-px  w-full bg-white bg-opacity-30" />
          </figure>
          {flowJson?.trigger?.icon ? <NodeArray flow={flowJson} /> : null}
        </div>
      </div>
    </Link>
  );
};

export default TemplateCard;

const NodeArray = ({ flow }: { flow: any }) => {
  //Loop through trigger and all actions to create icons
  const actions = [...flow.actions.map((action: any) => action.icon)];
  const visibleActions = actions.slice(0, 4);
  const hiddenIconsCount = actions.length - visibleActions.length;

  return (
    <div className="flex h-full flex-row gap-2">
      <BaseNodeIcon className="text-pink-500" icon={flow.trigger.icon} />
      <div className="flex h-14 items-center justify-center font-bold">
        {/* <VscArrowSmallRight className="w-6 text-3xl" /> */}
      </div>
      {visibleActions.map((icon, index) => {
        if (index === 3 && hiddenIconsCount > 0) {
          return (
            <div
              className="flex h-14 w-14 items-center justify-center rounded-md border p-2 text-xl opacity-50"
              key={index}
            >
              +{hiddenIconsCount + 1}
            </div>
          );
        }
        return <BaseNodeIcon icon={icon} key={index} />;
      })}
    </div>
  );
};
