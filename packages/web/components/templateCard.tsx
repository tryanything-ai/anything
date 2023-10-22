import Link from "next/link";
import { VscArrowSmallRight } from "react-icons/vsc";

import { FlowTemplate } from "@/types/flow";
import { Json, Tag } from "@/types/supabase.types";

import { AvatarAndUsername } from "./avatarAndUsername";
import BaseNodeIcon from "./baseNodeIcons";

export type CardProps = {
  slug: string;
  description: string;
  profile_name: string;
  profile: boolean;
  avatar_url: string;
  username: string;
  flow_name: string;
  flow_template_json: Json;
  tags: Tag[];
};

export function TemplateCard({
  flow_template_json,
  avatar_url,
  username,
  profile_name,
  profile,
  tags,
  slug,
  description,
  flow_name,
}: CardProps) {
  const flowJson =
    typeof flow_template_json === "string"
      ? JSON.parse(flow_template_json)
      : flow_template_json;

  return (
    <Link href={"/templates/" + slug}>
      <div className="card card-compact bg-base-300 overflow-hidden shadow-xl max-w-md sm:w-96 mx-1 transition-all duration-200 ease-in-out transform hover:scale-105">
        <div className="card-body">
          <h2 className="card-title text-2xl text-ellipsis">{flow_name}</h2>
          {/* User */}
          {profile ? (
            <AvatarAndUsername
              link={false}
              avatar_url={avatar_url}
              profile_name={profile_name}
              username={username}
            />
          ) : null}

          <p className="line-clamp-2 overflow-ellipsis overflow-hidden mb-2">
            {description}
          </p>
          <figure>
            <div className="h-px bg-white  w-full bg-opacity-30 mb-1" />
          </figure>
          {flowJson?.trigger?.icon && <NodeArray flow={flowJson} />}
        </div>
      </div>
    </Link>
  );
}

const NodeArray = ({ flow }: { flow: FlowTemplate }) => {
  //Loop through trigger and all actions to create icons
  const actions = [...flow.actions.map((action) => action.icon)];
  const visibleActions = actions.slice(0, 4);
  const hiddenIconsCount = actions.length - visibleActions.length;

  return (
    <div className="flex flex-row gap-2 h-full">
      <BaseNodeIcon icon={flow.trigger.icon} className="text-pink-500" />
      <div className="h-14 font-bold flex justify-center items-center">
        <VscArrowSmallRight className="w-6 text-3xl" />
      </div>
      {visibleActions.map((icon, index) => {
        if (index === 3 && hiddenIconsCount > 0) {
          return (
            <div
              className="h-14 w-14 p-2 text-xl rounded-md border opacity-50 flex justify-center items-center"
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
