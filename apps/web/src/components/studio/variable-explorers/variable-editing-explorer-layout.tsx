import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@repo/ui/components/ui/tabs";
import { useAnything } from "@/context/AnythingContext";
import { ScrollArea } from "@repo/ui/components/ui/scroll-area";
import { ResultsExplorer } from "./results-explorer";
import { useState } from "react";
import { Button } from "@repo/ui/components/ui/button";
import { XIcon } from "lucide-react";
import { SecretsExplorer } from "./secrets-explorer";
import { SystemVariablesExplorer } from "./system-variables-explorer";
import { cn } from "@repo/ui/lib/utils";
import { FilesExplorer } from "./files-explorer";
// Reusable tabs component
function VariableExplorerTabs({ className }: { className?: string }) {
  return (
    <TabsList className={cn("flex-1", className)}>
      <TabsTrigger value="results">Action Results</TabsTrigger>
      <TabsTrigger value="secrets">Secrets</TabsTrigger>
      <TabsTrigger value="files">Files</TabsTrigger>
      <TabsTrigger value="system_variables">System</TabsTrigger>
    </TabsList>
  );
}

// Reusable content component
function VariableExplorerContent() {
  return (
    <>
      <TabsContent value="results" className="h-full overflow-y-auto">
        <ScrollArea className="h-full">
          <ResultsExplorer />
        </ScrollArea>
      </TabsContent>
      <TabsContent value="secrets" className="h-full overflow-y-auto">
        <ScrollArea className="h-full">
          <SecretsExplorer />
        </ScrollArea>
      </TabsContent>
      <TabsContent value="files" className="h-full overflow-y-auto">
        <ScrollArea className="h-full">
          <FilesExplorer />
        </ScrollArea>
      </TabsContent>
      <TabsContent value="system_variables" className="h-full overflow-y-auto">
        <ScrollArea className="h-full">
          <SystemVariablesExplorer />
        </ScrollArea>
      </TabsContent>
    </>
  );
}

// Base component with just tabs and content
export function BaseVariableEditingExplorer(): JSX.Element {
  const [tab, setTab] = useState("results");

  return (
    <Tabs
      defaultValue="results"
      value={tab}
      onValueChange={setTab}
      className="flex flex-col h-full"
    >
      <VariableExplorerTabs className="w-[330px]" />
      <VariableExplorerContent />
    </Tabs>
  );
}

// Default export with close button and layout
export default function VariableEditingExplorer(): JSX.Element {
  const {
    workflow: { setShowExplorer },
  } = useAnything();
  const [tab, setTab] = useState("results");

  return (
    <div className="h-full">
      <Tabs
        defaultValue="results"
        value={tab}
        onValueChange={setTab}
        className="flex flex-col h-full p-2"
      >
        <div className="flex items-center gap-2">
          <Button
            onClick={() => setShowExplorer(false)}
            variant="outline"
            size="icon"
            aria-label="Close"
          >
            <XIcon className="size-5 fill-foreground" />
          </Button>
          <VariableExplorerTabs className="w-[330px]" />
        </div>
        <VariableExplorerContent />
      </Tabs>
    </div>
  );
}
