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
import { Send, XIcon } from "lucide-react";
import { SecretsExplorer } from "./secrets-explorer";

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
          <TabsList className="w-[200px]">
            <TabsTrigger value="results">Action Results</TabsTrigger>
            <TabsTrigger value="secrets">Secrets</TabsTrigger>
          </TabsList>
        </div>
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
      </Tabs>
    </div>
  );
}
