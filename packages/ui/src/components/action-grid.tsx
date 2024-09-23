import { cn } from "../lib/utils";
import { BaseNodeIcon } from "./baseNodeIcons";
import { Button } from "./ui/button";
import { EllipsisVertical } from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "./ui/dropdown-menu";

interface ActionTemplate {
  action_template_id: string;
  action_template_name: string;
  action_template_description: string;
  action_template_definition: any;
  type: string;
}

const ActionNode = ({ data }: { data: ActionTemplate }) => {
  return (
    <div
      className={cn(
        "bg-white border border-gray-300 text-primary-content flex h-20 w-90 flex-row rounded-md text-xl hover:bg-gray-50",
      )}
    >
      <div className="flex h-full w-full flex-row items-center p-3">
        <BaseNodeIcon icon={data.action_template_definition.icon} />
        <div className="flex flex-col">
          <div className="px-4">{data.action_template_name}</div>
          <div className="px-4 text-sm font-light">
            {data.action_template_description}
          </div>
        </div>
      </div>
    </div>
  );
};

export const ActionTemplateGrid = ({
  actionTemplates,
}: {
  actionTemplates: ActionTemplate[];
}) => {
  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {actionTemplates.map((template) => (
        <ActionNode key={template.action_template_id} data={template} />
      ))}
    </div>
  );
};
