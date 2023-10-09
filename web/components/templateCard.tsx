import Link from "next/link";
import { Database } from "@/types/supabase.types";
import BaseNodeIcon from "./baseNodeIcons";
import { Flow as LocalFlow } from "../../tauri/src/utils/newNodes";
import clsx from "clsx";

type Flow = Database["public"]["Tables"]["flow_templates"]["Row"];

export async function TemplateCard({ template }: { template: Flow }) {
  const flowJson =
    typeof template.flow_json === "string"
      ? JSON.parse(template.flow_json)
      : template.flow_json;

  return (
    <Link
      // className="cfard bg-base-300 shadow-xl my-2"
      // as={Link}
      // image={
      //   <figure>
      //     <Image
      //       src=""
      //       alt={`${template.flow_name} descriptive image` || "Template Image"}
      //     />
      //   </figure>
      // }
      href={template.slug}
    >
      <div className="card card-compact bg-base-300 overflow-hidden shadow-xl w-96 transition-all duration-200 ease-in-out transform hover:scale-105">
        <div className="card-body">
          <h2 className="card-title text-2xl text-ellipsis">
            {template.flow_name}
            {/* <div className="badge badge-secondary">NEW</div> */}
          </h2>
          <p>{flowJson.description}</p>
          {/* //TODO: add tags from supabase. make them clickable */}
          {/* <div className="card-actions justify-end">
          <div className="badge badge-outline">Fashion</div>
          <div className="badge badge-outline">Products</div>
        </div> */}
          {flowJson?.trigger?.icon && <NodeArray flow={flowJson} />}
        </div>
      </div>
    </Link>
  );
}

export const NodeArray = ({ flow }: { flow: LocalFlow }) => {
  //Loop through trigger and all actions to create icons
  const icons = [
    flow.trigger.icon,
    ...flow.actions.map((action) => action.icon),
  ];
  const visibleIcons = icons.slice(0, 4);
  const hiddenIconsCount = icons.length - visibleIcons.length;

  return (
    <div className="flex flex-row gap-2">
      {visibleIcons.map((icon, index) => (
        <BaseNodeIcon key={index} icon={icon} className={clsx({ 'text-pink-500': index === 0 })} />
      ))}
      {hiddenIconsCount > 0 && <span>+{hiddenIconsCount}</span>}
    </div>
  );
  // return <BaseNodeIcon icon={flow.trigger.icon} />;
};
