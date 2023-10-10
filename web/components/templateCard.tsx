import Link from "next/link";
import { Database } from "@/types/supabase.types";
import BaseNodeIcon from "./baseNodeIcons";
import { Flow as LocalFlow } from "../../tauri/src/utils/newNodes";
import { VscArrowSmallRight } from "react-icons/vsc";

type Flow = Database["public"]["Tables"]["flow_templates"]["Row"];

export function TemplateCard({ template }: { template: Flow }) {
  const flowJson =
    typeof template.flow_json === "string"
      ? JSON.parse(template.flow_json)
      : template.flow_json;

  return (
    <Link
      // image={
      //   <figure>
      //     <Image
      //       src=""
      //       alt={`${template.flow_name} descriptive image` || "Template Image"}
      //     />
      //   </figure>
      // }
      href={"/templates/" + template.slug}
    >
      <div className="card card-compact bg-base-300 overflow-hidden shadow-xl max-w-md sm:w-96 mx-2 transition-all duration-200 ease-in-out transform hover:scale-105">
        <div className="card-body">
          <h2 className="card-title text-2xl text-ellipsis">
            {template.flow_name}
            {/* <div className="badge badge-secondary">NEW</div> */}
          </h2>
          <div className="mb-2 flex gap-1">
            <div className="badge badge-outline">Fashion</div>
            <div className="badge badge-outline">Products</div>
          </div>
          <p className="line-clamp-2 overflow-ellipsis overflow-hidden mb-2">
            {flowJson.description}
          </p>
          {/* //TODO: add tags from supabase. make them clickable */}

          <figure>
            <div className="h-px bg-white  w-full bg-opacity-30 mb-1" />
          </figure>
          {flowJson?.trigger?.icon && <NodeArray flow={flowJson} />}
        </div>
      </div>
    </Link>
  );
}

export const NodeArray = ({ flow }: { flow: LocalFlow }) => {
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
