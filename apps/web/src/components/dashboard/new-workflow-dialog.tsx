"use client";

import { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@repo/ui/components/ui/dialog";
import { Button } from "@repo/ui/components/ui/button";
import { Label } from "@repo/ui/components/ui/label";
import { Input } from "@repo/ui/components/ui/input";
import { Textarea } from "@repo/ui/components/ui/textarea";
import api from "@repo/anything-api";
import { useRouter } from "next/navigation";
import { useAnything } from "@/context/AnythingContext";
import FieldJson from "@/components/studio/forms/fields/field-json";

interface NewWorkflowDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onCreateWorkflow: (name: string, description: string) => Promise<void>;
}

export default function NewWorkflowDialog({
  open,
  onOpenChange,
  onCreateWorkflow,
}: NewWorkflowDialogProps) {
  const router = useRouter();

  const {
    accounts: { selectedAccount },
  } = useAnything();

  const [workflowJson, setWorkflowJson] = useState("{}");
  const [isValidJson, setIsValidJson] = useState(true);
  const [showJsonInput, setShowJsonInput] = useState(false);
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");

  const createWorkflowFromJson = async () => {
    if (!selectedAccount) {
      console.error("No account selected");
      return;
    }

    if (!name || name.trim() === "") {
      console.error("Workflow name cannot be empty");
      return;
    }

    if (!isValidJson) {
      alert("Please enter valid JSON before submitting");
      return;
    }

    try {
      // Parse JSON one final time to ensure validity
      const parsedJson = JSON.parse(workflowJson);

      let res = await api.flows.createFlowFromJson(
        selectedAccount.account_id,
        name,
        parsedJson,
      );

      //On response navigate to the new workflow
      router.push(
        `/workflows/${res.workflow_id}/${res.workflow_version_id}/editor`,
      );
    } catch (error) {
      console.error("error creating workflow from json", error);

      // Extract error message from API response
      let errorMessage = "Failed to create workflow";
      if (error instanceof Error) {
        if ("response" in error && error.response) {
          // @ts-ignore
          errorMessage = error.response.data || error.message;
        } else {
          errorMessage = error.message;
        }
      }

      // Show error alert to user
      alert(`Error creating workflow: ${errorMessage}`);
    }
  };

  const handleJsonChange = (_name: string, value: string, valid: boolean) => {
    setWorkflowJson(value);
    setIsValidJson(valid);
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent
        className={
          showJsonInput ? "w-4/5 h-4/5 max-w-none overflow-y-auto" : "w-1/2 max-w-none"
        }
      >
        <DialogHeader>
          <div className="flex flex-row justify-between pr-20">
            <div className="flex flex-col">
              <DialogTitle>Create New Workflow</DialogTitle>
              <DialogDescription>
                Create a new workflow or upload workflow json
              </DialogDescription>
            </div>

            <div className="mt-4">
              <Button variant="outline" onClick={() => setShowJsonInput(true)}>
                Import from JSON
              </Button>
            </div>
          </div>
        </DialogHeader>

        <div className="flex flex-col space-y-4 p-4">
          <div className="space-y-2">
            <Label htmlFor="name">Workflow Name</Label>
            <Input
              id="name"
              placeholder="Enter workflow name"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>

          {showJsonInput ? (
            <>
              <div className="flex justify-end space-x-2 mb-4">
                <Button
                  variant="outline"
                  onClick={() => setShowJsonInput(false)}
                >
                  Cancel
                </Button>
                <Button
                  onClick={() => createWorkflowFromJson()}
                  disabled={!name.trim() || !isValidJson}
                >
                  Import
                </Button>
              </div>
              <p className="text-yellow-600 font-semibold">
                Experimental: Providing incorrect JSON will cause problems. Use
                at your own risk. 🐉
              </p>
              <div className="h-[calc(100vh-400px)] min-h-[300px]">
                <FieldJson
                  className="h-full resize-none border border-gray-300 visible overflow-auto"
                  name="workflow-json"
                  label="Paste your JSON here:"
                  value={workflowJson}
                  onChange={handleJsonChange}
                  isVisible={true}
                />
              </div>
            </>
          ) : (
            <>
              <div className="space-y-2">
                <Label htmlFor="description">Description</Label>
                <Textarea
                  id="description"
                  placeholder="Enter workflow description"
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                />
              </div>
              <div className="flex justify-center">
                <Button
                  size="lg"
                  onClick={() => onCreateWorkflow(name, description)}
                  disabled={!name.trim()}
                >
                  Create Workflow
                </Button>
              </div>
            </>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
