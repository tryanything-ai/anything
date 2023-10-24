import { FlowTemplate, Json } from "@anything/utils";
import React from "react";
import { VscArrowSmallRight } from "react-icons/vsc";

import { AvatarAndUsername } from "./avatarAndUsername";
import BaseNodeIcon from "./baseNodeIcons";

export type TemplateCardProps = {
  // next?: boolean;
  slug: string;
  description: string;
  profile_name: string;
  profile: boolean;
  // avatar_url: string;
  username: string;
  flow_name: string;
  flow_template_json: Json;
  // tags: Tag[];
  Link: React.ComponentType<any>;
  AvatarComponent: React.ComponentType;
};

const TemplateCard = ({
  flow_template_json,
  // avatar_url,
  username,
  profile_name,
  profile,
  // tags,
  slug,
  description,
  flow_name,
  Link,
  AvatarComponent,
}: TemplateCardProps) => {
  const flowJson =
    typeof flow_template_json === "string"
      ? JSON.parse(flow_template_json)
      : flow_template_json;

  return (
    <Link href={"/templates/" + slug} to={"/templates/" + slug}>
      <div className="card card-compact bg-base-300 mx-1 max-w-md transform overflow-hidden shadow-xl transition-all duration-200 ease-in-out hover:scale-105 sm:w-96">
        <div className="card-body">
          <h2 className="card-title text-ellipsis text-2xl">{flow_name}</h2>
          {/* User */}
          {profile ? (
            <AvatarAndUsername
              AvatarComponent={AvatarComponent}
              Link={Link}
              link={false}
              profile_name={profile_name}
              username={username}
            />
          ) : null}

          <p className="mb-2 line-clamp-2 overflow-hidden overflow-ellipsis">
            {description}
          </p>
          <figure>
            <div className="mb-1 h-px  w-full bg-white bg-opacity-30" />
          </figure>
          {flowJson?.trigger?.icon && <NodeArray flow={flowJson} />}
        </div>
      </div>
    </Link>
  );
};

export default TemplateCard;

const NodeArray = ({ flow }: { flow: FlowTemplate }) => {
  //Loop through trigger and all actions to create icons
  const actions = [...flow.actions.map((action) => action.icon)];
  const visibleActions = actions.slice(0, 4);
  const hiddenIconsCount = actions.length - visibleActions.length;

  return (
    <div className="flex h-full flex-row gap-2">
      <BaseNodeIcon icon={flow.trigger.icon} className="text-pink-500" />
      <div className="flex h-14 items-center justify-center font-bold">
        <VscArrowSmallRight className="w-6 text-3xl" />
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
        } else {
          return <BaseNodeIcon key={index} icon={icon} />;
        }
      })}
    </div>
  );
};
