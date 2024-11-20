import { cn } from "@/lib/utils";
import { BaseNodeIcon } from "@/components/studio/nodes/node-icon";
import { Handle, HandleProps } from "reactflow";
import { Action, ActionType } from "@/types/workflows";
import { EllipsisVertical } from "lucide-react";
import { Button } from "@repo/ui/components/ui/button";
import { useState } from "react";

import { DropdownMenuCheckboxItemProps } from "@radix-ui/react-dropdown-menu";

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@repo/ui/components/ui/dropdown-menu";
import { useAnything } from "@/context/AnythingContext";
import PublishActionDialog from "../publish-action-dialog";

type Checked = DropdownMenuCheckboxItemProps["checked"];

export default function BaseNode({
  id,
  data,
  selected,
}: {
  id: string;
  data: Action;
  selected: boolean;
}): JSX.Element {
  const {
    workflow: { deleteNode, detailedMode, addNode, showActionSheetToChangeTrigger },
  } = useAnything();

  const [showDialog, setShowDialog] = useState(false);

  const chooseOtherTrigger = () => { 
    showActionSheetToChangeTrigger()
  }

  const handleButtonClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.stopPropagation();
    // Handle button click logic here
    console.log("Node data:", data);
  };


  const duplicateAction = (event: React.MouseEvent<HTMLDivElement>) => {
    event.stopPropagation();
    console.log("TODO: Make duplicate action");
    addNode(data, { x: 100, y: 300 });
  };

  const downloadJson = (event: React.MouseEvent<HTMLDivElement>) => {
    event.stopPropagation();
    console.log("TODO: Download JSON");

    event.stopPropagation();
    const jsonString = JSON.stringify(data, null, 2);
    const blob = new Blob([jsonString], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = `${data.label || "action"}.json`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  };

  const shareAction = (event: React.MouseEvent<HTMLDivElement>) => {
    setShowDialog(true);
  };

  const deleteAction = (event: React.MouseEvent<HTMLDivElement>) => {
    event.stopPropagation();
    console.log("TODO: Delete Action");
    deleteNode(id);
  };

  const copyToClipboard = (event: React.MouseEvent<HTMLDivElement>) => {
    event.stopPropagation();
    const jsonString = JSON.stringify(data, null, 2);
    navigator.clipboard
      .writeText(jsonString)
      .then(() => {
        console.log("Data copied to clipboard");
      })
      .catch((err) => {
        console.error("Failed to copy data: ", err);
      });
  };

  return (
    <>
      <PublishActionDialog
        show={showDialog}
        onClose={() => setShowDialog(false)}
      />
      <DropdownMenu>
        <div
          // onClick={toggleNodeConfig}
          className={cn(
            "bg-white border border-gray-300 text-primary-content flex h-20 w-90 flex-row rounded-md text-xl hover:bg-gray-50",
            selected ? "border-pink-700" : "",
          )}
        >
          {data.handles
            ? data.handles.map((handle: HandleProps) => {
                return (
                  <Handle
                    key={handle.id}
                    type={handle.type}
                    position={handle.position}
                    id={handle.id}
                  />
                );
              })
            : null}

          {/* Container */}
          <div className="flex h-full w-full flex-row items-center p-3">
            <BaseNodeIcon icon={data.icon} />
            <div className="flex flex-col">
              <div className="px-4">{data.label}</div>
              {/* {detailedMode && data.description && (<div className="text-sm">{data.description}</div>)} */}
              {detailedMode && data.action_id && (
                <div className=" px-4 text-sm font-light">{data.action_id}</div>
              )}
            </div>
          </div>
          <div className="flex h-full flex-row items-center pr-3">
            {data.type === ActionType.Trigger ? (
              <DropdownMenuTrigger asChild>
                <Button
                  variant="outline"
                  className="p-2"
                  onClick={handleButtonClick}
                >
                  <EllipsisVertical className="w-4" />
                </Button>
              </DropdownMenuTrigger>
            ) : (
              <DropdownMenuTrigger asChild>
                <Button
                  variant="outline"
                  className="p-2"
                  onClick={handleButtonClick}
                >
                  <EllipsisVertical className="w-4" />
                </Button>
              </DropdownMenuTrigger>
            )}
          </div>
        </div>
        {/* Content of Dropdown for non-trigger nodes */}
        {data.type !== ActionType.Trigger && (
          <DropdownMenuContent className="w-56">
            <DropdownMenuItem onClick={duplicateAction}>
              Duplicate
            </DropdownMenuItem>
            <DropdownMenuItem onClick={shareAction}>
              Make Reusable Action
            </DropdownMenuItem>
            <DropdownMenuItem onClick={downloadJson}>
              Download as JSON
            </DropdownMenuItem>
            <DropdownMenuItem onClick={copyToClipboard}>
              Copy to Clipboard
            </DropdownMenuItem>
            <DropdownMenuItem onClick={deleteAction}>Delete</DropdownMenuItem>
          </DropdownMenuContent>
        )}
        {/* Content of Dropdown for trigger nodes */}
        {data.type === ActionType.Trigger && (
          <DropdownMenuContent className="w-56">
            <DropdownMenuItem onClick={chooseOtherTrigger}>
              Choose Different Trigger
            </DropdownMenuItem>
          </DropdownMenuContent>
        )}
      </DropdownMenu>
    </>
  );
}
