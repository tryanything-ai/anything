
"use client"

import { z } from "zod"
import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"

const formSchema = z.object({
    secret_name: z.string().min(2).max(50),
    secret_value: z.string().min(2).max(50),
    secret_description: z.string().min(2).max(50),
})

import { Button } from "@/components/ui/button"
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form"
import { Input } from "@/components/ui/input"

import { useState } from "react";
import { Edit2, Trash2 } from "lucide-react";

export function CreateNewSecret({ cancel, saveSecret }: any) {

    // 1. Define your form.
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            secret_name: "",
            secret_value: "",
            secret_description: "",
        },
    })

    // 2. Define a submit handler.
    function onSubmit(values: z.infer<typeof formSchema>) {
        // Do something with the form values.
        // ✅ This will be type-safe and validated.
        console.log(values)
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
                                        <FormLabel>Name</FormLabel>
                                        <FormControl>
                                            <Input placeholder="OPENAI_API_KEY" {...field} />
                                        </FormControl>
                                        <FormDescription>
                                            This is the public name you will use to refer to this secret.
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                control={form.control}
                                name="secret_value"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>Value</FormLabel>
                                        <FormControl>
                                            <Input placeholder="pk_0498234i-0i" {...field} />
                                        </FormControl>
                                        <FormDescription>
                                            Stored enrypted in the database for use in your automations.
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
                                        <Input placeholder="Our api key for interacting with ..." {...field} />
                                    </FormControl>
                                    <FormDescription>
                                        One line note to help you recall how this is used.
                                    </FormDescription>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <Button variant={"secondary"} className="mr-2" onClick={cancel}>Cancel</Button>
                        <Button type="submit">Save</Button>
                    </form>
                </Form>
            </div>
        </div>
    )
}


export function EditSecret({ secret, deleteSecret, updateSecret }: any) {

    const [editing, setEditing] = useState(false);

    // 1. Define your form.
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            secret_name: "",
            secret_value: "",
            secret_description: "",
        },
    })

    // 2. Define a submit handler.
    function onSubmit(values: z.infer<typeof formSchema>) {
        // Do something with the form values.
        // ✅ This will be type-safe and validated.
        console.log(values)
    }

    return (
        <div className="flex flex-row max-w-5xl mx-auto">
            {secret && !editing ? (
                <div className="flex-1 flex flex-row">

                    <div className="flex flex-col flex-1">
                        <div className="text-lg font-semibold mr-2">{secret.secret_name}</div>
                        <div className="h-10 font-light">{secret.secret_description}</div>
                    </div>
                    <div className="flex flex-col flex-1">
                        <div>{secret.secret_value}</div>
                    </div>

                    {/* <Input type="" value={secret.secret_value} readOnly /> */}
                    <Button variant="outline" size="sm" className="ml-2" onClick={() => setEditing(!editing)}>
                        <Edit2 className="size-5" />
                    </Button>
                    <Button variant="outline" size="sm" className="ml-2" onClick={() => openDialog(secret)}>
                        <Trash2 className="size-5" />
                    </Button>
                </div>
            ) : (
                <div className="flex-1 flex flex-row max-w-5xl mx-auto border p-7 rounded-md m-7">
                    <Form {...form}>
                        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
                            <div className="flex flex-row">
                                <FormField
                                    control={form.control}
                                    name="secret_name"
                                    render={({ field }) => (
                                        <FormItem className="mr-10">
                                            <FormLabel>Name</FormLabel>
                                            <FormControl>
                                                <Input placeholder="OPENAI_API_KEY" {...field} />
                                            </FormControl>
                                            <FormDescription>
                                                This is the public name you will use to refer to this secret.
                                            </FormDescription>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                                <FormField
                                    control={form.control}
                                    name="secret_value"
                                    render={({ field }) => (
                                        <FormItem>
                                            <FormLabel>Value</FormLabel>
                                            <FormControl>
                                                <Input placeholder="pk_0498234i-0i" {...field} />
                                            </FormControl>
                                            <FormDescription>
                                                Stored enrypted in the database for use in your automations.
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
                                            <Input placeholder="Our api key for interacting with ..." {...field} />
                                        </FormControl>
                                        <FormDescription>
                                            One line note to help you recall how this is used.
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <Button variant={"secondary"} className="mr-2" onClick={() => setEditing(false)}>Cancel</Button>
                            <Button type="submit">Save</Button>
                        </form>
                    </Form>
                </div>
            )
            }
        </div>
    )
}