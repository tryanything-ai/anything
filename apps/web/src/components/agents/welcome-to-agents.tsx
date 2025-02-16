import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import { Button } from "@repo/ui/components/ui/button";
import { PhoneCall, Puzzle } from "lucide-react";

const WelcomeToAgents = ({
  setShowCreator,
}: {
  setShowCreator: (show: boolean) => void;
}) => {
  return (
    <div className="container max-w-6xl py-6">
      <div className="mb-8">
        <h1 className="text-3xl font-bold tracking-tight">Voice Agents</h1>
        <p className="text-muted-foreground mt-2">
          Create AI-powered voice agents to handle your phone calls 24/7
        </p>
      </div>

      <div className="grid gap-6">
        <Card>
          <CardHeader>
            <CardTitle>Create Your First Voice Agent</CardTitle>
            <CardDescription>
              Get started by creating a customized AI voice agent that can
              handle calls on your behalf
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="flex flex-col items-start gap-4">
              <div className="grid gap-2">
                <div className="flex items-center gap-2">
                  <div className="size-9 rounded-full bg-primary/10 flex items-center justify-center">
                    <PhoneCall className="size-4 text-primary" />
                  </div>
                  <p className="text-sm text-muted-foreground">
                    Get a dedicated phone number for your agent
                  </p>
                </div>
                <div className="flex items-center gap-2">
                  <div className="size-9 rounded-full bg-primary/10 flex items-center justify-center">
                    <span className="text-primary text-sm">AI</span>
                  </div>
                  <p className="text-sm text-muted-foreground">
                    Powered by advanced AI to handle natural conversations
                  </p>
                </div>
                <div className="flex items-center gap-2">
                  <div className="size-9 rounded-full bg-primary/10 flex items-center justify-center">
                    <span className="text-primary text-sm">24/7</span>
                  </div>
                  <p className="text-sm text-muted-foreground">
                    Available around the clock to take calls
                  </p>
                </div>
                <div className="flex items-center gap-2">
                  <div className="size-9 rounded-full bg-primary/10 flex items-center justify-center">
                    <Puzzle className="size-4 text-primary" />
                  </div>
                  <p className="text-sm text-muted-foreground">
                    Integrate with your existing tools
                  </p>
                </div>
              </div>

              <Button onClick={() => setShowCreator(true)}>
                Create Voice Agent
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
};

export default WelcomeToAgents;
