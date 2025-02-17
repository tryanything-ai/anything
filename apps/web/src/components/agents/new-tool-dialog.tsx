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
import { useAnything } from "@/context/AnythingContext";

interface NewToolDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onCreateTool: (name: string, description: string) => Promise<void>;
}

export default function NewToolDialog({
  open,
  onOpenChange,
  onCreateTool,
}: NewToolDialogProps) {
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const [name, setName] = useState("");
  const [description, setDescription] = useState("");

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="w-1/2 max-w-none">
        <DialogHeader>
          <div className="flex flex-col">
            <DialogTitle>Create New Tool</DialogTitle>
            <DialogDescription>
              Create a new tool for your agent
            </DialogDescription>
          </div>
        </DialogHeader>

        <div className="flex flex-col space-y-4 p-4">
          <div className="space-y-2">
            <Label htmlFor="name">Tool Name</Label>
            <Input
              id="name"
              placeholder="Enter tool name"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="description">Description</Label>
            <Textarea
              id="description"
              placeholder="Enter tool description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
            />
          </div>
          <div className="flex justify-center">
            <Button
              size="lg"
              onClick={() => onCreateTool(name, description)}
              disabled={!name.trim()}
            >
              Create Tool
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
