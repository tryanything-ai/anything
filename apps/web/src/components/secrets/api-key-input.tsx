"use client";

import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";

import { Button } from "@repo/ui/components/ui/button";
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

//Use same schema for both
const formSchema = z.object({
  secret_name: z.string().max(100),
  secret_description: z.string().max(100).optional(),
});

export function CreateNewApiKey({ cancel, saveSecret }: any): JSX.Element {
  // 1. Define your form.
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      secret_name: "",
      secret_description: "",
    },
  });

  // 2. Define a submit handler.
  async function onSubmit(values: z.infer<typeof formSchema>) {
    // console.log(values)
    await saveSecret(values.secret_name, values.secret_description);
    form.reset();
    cancel();
  }

  return (
    <div className="flex flex-row max-w-5xl mx-auto">
      <div className="flex-1 flex flex-row max-w-5xl mx-auto border p-7 rounded-md m-7">
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
            <div className="flex flex-row">
              <FormField
                control={form.control}
                name="secret_name"
                render={({ field }) => (
                  <FormItem className="mr-10">
                    <FormLabel>API Key Name</FormLabel>
                    <FormControl>
                      <Input placeholder="Internal Tool API Key" {...field} />
                    </FormControl>
                    <FormDescription>
                      Simple name for this secret
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
            </div>

            <FormField
              control={form.control}
              name="secret_description"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Description</FormLabel>
                  <FormControl>
                    <Input
                      placeholder="Our api key for interacting with ..."
                      {...field}
                    />
                  </FormControl>
                  <FormDescription>
                    One line note to help recall where this is used.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button variant={"secondary"} className="mr-2" onClick={cancel}>
              Cancel
            </Button>
            <Button type="submit">Create</Button>
          </form>
        </Form>
      </div>
    </div>
  );
}
