import Link from "next/link";
import { Database } from "@/types/supabase.types";
import BaseNodeIcon from "./baseNodeIcons";
import BaseCard from "./baseCard";
// import Image from "next/image";
import { Flow as LocalFlow } from "../../tauri/src/utils/newNodes";

type Flow = Database["public"]["Tables"]["flow_templates"]["Row"];

export async function TemplateCard({ template }: { template: Flow }) {
  const flowJson =
    typeof template.flow_json === "string"
      ? JSON.parse(template.flow_json)
      : template.flow_json;

  return (
    <BaseCard
      as={Link}
      // image={
      //   <figure>
      //     <Image
      //       src=""
      //       alt={`${template.flow_name} descriptive image` || "Template Image"}
      //     />
      //   </figure>
      // }
      href={template.slug}
      className="m-2 w-72 transition-all duration-200 ease-in-out transform hover:scale-105"
    >
      <div className="card-body">
        <h2 className="card-title">
          {template.flow_name}
          {/* <div className="badge badge-secondary">NEW</div> */}
        </h2>
        <p>If a dog chews shoes whose shoes does he choose?</p>
        <div className="card-actions justify-end">
          <div className="badge badge-outline">Fashion</div>
          <div className="badge badge-outline">Products</div>
        </div>
      </div>
      {flowJson?.trigger?.icon && <NodeArray flow={flowJson} />}

      <div className="p-6">
        {/* <h2 className="text-lg font-semibold text-gray-700">{template.f}</h2> */}
        {/* <p className="text-gray-500">{template}</p> */}
      </div>
    </BaseCard>
  );
}

export const NodeArray = ({ flow }: { flow: LocalFlow }) => {
  //Loop through trigger and all actions to create icons
  return <BaseNodeIcon icon={flow.trigger.icon} />;
};
