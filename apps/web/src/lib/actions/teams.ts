'use server'

import { redirect } from "next/navigation";
import { createClient } from "../supabase/server";

export async function createTeam(prevState: any, formData: FormData) {
    "use server";

    const name = formData.get("name") as string;
    const slug = formData.get("slug") as string;
    const supabase = await createClient();

    const { data, error }: any = await supabase.rpc('create_team',
         // @ts-ignore
        {
        name,
        slug,
    } as any);

    if (error) {
        return {
            message: error.message
        };
    }

    // redirect(`/`);
};


export async function editTeamName(prevState: any, formData: FormData) {
    "use server";

    const name = formData.get("name") as string;
    const accountId = formData.get("accountId") as string;
    const supabase = await createClient();

    const { error }: any = await supabase.rpc('update_account', 
         // @ts-ignore
        {
        name,
        account_id: accountId
    } as any);

    if (error) {
        return {
            message: error.message
        };
    }
};

export async function editTeamSlug(prevState: any, formData: FormData) {
    "use server";

    const slug = formData.get("slug") as string;
    const accountId = formData.get("accountId") as string;
    const supabase = await createClient();

    const { data, error }: any = await supabase.rpc('update_account', 
         // @ts-ignore
        {
        slug,
        account_id: accountId
    } as any);

    if (error) {
        return {
            message: error.message
        };
    }

    redirect(`/dashboard/${data.slug}/settings`);
};