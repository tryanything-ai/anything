const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const getProfilesFromMarketplace = async () => {
    try {
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        // const supabase = createClient();
        // const { data: { session } } = await supabase.auth.getSession();

        // console.log('Session:', session);

        // const headers: HeadersInit = {
        //     'Content-Type': 'application/json',
        // };

        // if (session) {
        //     headers['Authorization'] = `${session.access_token}`;
        // }

        const response = await fetch(`${ANYTHING_API_URL}/marketplace/profiles`);
        const data = await response.json();
        console.log('Data from /api/marketplace/profiles:', data);
        return data;
    } catch (error) {
        console.error('Error fetching profiles:', error);
    }
}

export const getMarketplaceProfileByUsername = async (username: string) => {
    try {
        // const supabase = createClient();
        // const { data: { session } } = await supabase.auth.getSession();

        // console.log('Session:', session);

        // const headers: HeadersInit = {
        //     'Content-Type': 'application/json',
        // };

        // if (session) {
        //     headers['Authorization'] = `${session.access_token}`;
        // }

        const response = await fetch(`${ANYTHING_API_URL}/marketplace/profile/${username}`);
        const data = await response.json();
        console.log(`Data from /api/marketplace/profile/${username}:`, data);
        return data;
    } catch (error) {
        console.error('Error fetching profile by username:', error);
    }
}