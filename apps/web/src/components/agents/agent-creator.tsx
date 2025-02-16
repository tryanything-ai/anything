"use client";

import { useState } from "react";
import { Button } from "@repo/ui/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@repo/ui/components/ui/form";
import { Input } from "@repo/ui/components/ui/input";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import * as z from "zod";
import { useRouter } from "next/navigation";
import api from "@repo/anything-api";
import { useAnything } from "@/context/AnythingContext";
import { createClient } from "@/lib/supabase/client";

const formSchema = z.object({
  name: z.string().min(2, {
    message: "Name must be at least 2 characters.",
  }),
});

export default function AgentCreator() {
  const router = useRouter();
  const [isLoading, setIsLoading] = useState(false);
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: "",
    },
  });

  const onSubmit = async (values: z.infer<typeof formSchema>) => {
    if (!selectedAccount) {
      console.error("No account selected");
      return;
    }

    if (!values.name || values.name.trim() === "") {
      console.error("Agent name cannot be empty");
      return;
    }

    try {
      let res = await api.agents.createAgent(
        await createClient(),
        selectedAccount.account_id,
        values.name.trim(),
      );
      console.log("created  agent", res);
      router.push(`/agents/${res.agent_id}`);
    } catch (error) {
      console.error("error creating agent", error);
    }
  };

  return (
    <div className="container max-w-2xl py-6">
      <Card>
        <CardHeader>
          <CardTitle>Create a New Voice Agent</CardTitle>
          <CardDescription>
            Configure your AI-powered voice agent to handle calls on your behalf
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
              <FormField
                control={form.control}
                name="name"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Agent Name</FormLabel>
                    <FormControl>
                      <Input placeholder="Customer Service Agent" {...field} />
                    </FormControl>
                    <FormDescription>
                      A name to identify your agent
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <Button type="submit" className="w-full" disabled={isLoading}>
                {isLoading ? "Creating..." : "Create Voice Agent"}
              </Button>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}
