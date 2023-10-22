import { env } from "@/env.mjs";
import { SiteConfig } from "@/types";

export const siteConfig: SiteConfig = {
  name: "Anything",
  description:
    "The Easiest way to automate your business with AI",
  url: "https://" + env.NEXT_PUBLIC_VERCEL_URL,
  ogImage: `https://${env.NEXT_PUBLIC_VERCEL_URL}/og.jpg`,
  links: {
    twitter: "https://twitter.com/carllippert",
    github: "https://github.com/tryanything-ai/anything",
  },
};
