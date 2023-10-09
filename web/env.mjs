import { createEnv } from "@t3-oss/env-nextjs";
import { z } from "zod";

export const env = createEnv({
  server: {
    // SUPABASE_PROJECT_ID: process.env.SUPABASE_PROJECT_ID,
    // NEXTAUTH_URL: z.string().url().optional(),
    // NEXTAUTH_SECRET: z.string().min(1),
    // GITHUB_CLIENT_ID: z.string().min(1),
    // GITHUB_CLIENT_SECRET: z.string().min(1),
    // TIER_BASE_URL: z.string().min(1),
    // TIER_API_KEY: z.string().min(1),
    // OPENAI_API_KEY: z.string().min(1),
    // POSTGRES_PRISMA_URL: z.string().min(1),
    // POSTGRES_URL_NON_POOLING: z.string().min(1),
  },
  client: {
    NEXT_PUBLIC_APP_URL: z.string().min(1),
    // SUPABASE_PROJECT_ID: z.string().min(1),
    NEXT_PUBLIC_SUPABASE_URL: z.string().min(1),
    NEXT_PUBLIC_SUPABASE_ANON_KEY: z.string().min(1),
    NEXT_PUBLIC_POSTHOG_KEY: z.string().min(1),
    NEXT_PUBLIC_POSTHOG_HOST: z.string().min(1),
  },
  runtimeEnv: {
    NEXT_PUBLIC_APP_URL: process.env.NEXT_PUBLIC_APP_URL,
    NEXT_PUBLIC_SUPABASE_URL: process.env.NEXT_PUBLIC_SUPABASE_URL,
    NEXT_PUBLIC_SUPABASE_ANON_KEY: process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY,
    NEXT_PUBLIC_POSTHOG_KEY: process.env.NEXT_PUBLIC_POSTHOG_KEY,
    NEXT_PUBLIC_POSTHOG_HOST: process.env.NEXT_PUBLIC_POSTHOG_HOST,
    // NEXTAUTH_URL: process.env.NEXTAUTH_URL,
    // NEXTAUTH_SECRET: process.env.NEXTAUTH_SECRET,
    // GITHUB_CLIENT_ID: process.env.GITHUB_CLIENT_ID,
    // GITHUB_CLIENT_SECRET: process.env.GITHUB_CLIENT_SECRET,
    // TIER_BASE_URL: process.env.TIER_BASE_URL,
    // TIER_API_KEY: process.env.TIER_API_KEY,
    // OPENAI_API_KEY: process.env.OPENAI_API_KEY,
    // POSTGRES_PRISMA_URL: process.env.POSTGRES_PRISMA_URL,
    // POSTGRES_URL_NON_POOLING: process.env.POSTGRES_URL_NON_POOLING,
  },
});
