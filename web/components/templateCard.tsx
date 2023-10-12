import Link from "next/link";
import BaseNodeIcon from "./baseNodeIcons";
import { Flow as LocalFlow } from "../../tauri/src/utils/newNodes";
import { VscArrowSmallRight } from "react-icons/vsc";
import { Json, Tag } from "@/types/supabase.types";
import Image from "next/image";

export type CardProps = {
  slug: string;
  description: string;
  profile_name: string;
  profile: boolean;
  avatar_url: string;
  flow_name: string;
  flow_template_json: Json;
  tags: Tag[];
};

export function TemplateCard({
  flow_template_json,
  avatar_url,
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
            <div className="flex flex-row">
              <div className="avatar">
                <div className="w-10 rounded-full">
                  <Image
                    width={100}
                    height={100}
                    src={avatar_url}
                    alt={profile_name}
                  />
                </div>
              </div>
              <div className="flex flex-col pl-4 justify-center">
                <div className="text-ellipsis">{profile_name}</div>
                {/* <div className="opacity-70">20 templates</div> */}
              </div>
            </div>
          ) : null}

          {/* {tags ? (
            <div className="mb-2 flex gap-1">
              {tags.map((tag, index) => {
                return (
                  <div className="badge badge-outline" key={index}>
                    {tag.tag_label}
                  </div>
                );
              })}
            </div>
          ) : null} */}

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

const NodeArray = ({ flow }: { flow: LocalFlow }) => {
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
