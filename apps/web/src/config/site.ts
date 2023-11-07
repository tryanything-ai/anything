import { SiteConfig } from "@/types";

export const siteConfig: SiteConfig = {
  name: "Anything AI",
  description:
    "The easiest way to automate your business",
  url: `https://${process.env.NEXT_PUBLIC_VERCEL_URL}`,
  ogImage: `https://${process.env.NEXT_PUBLIC_VERCEL_URL}/og.jpg`,
  links: {
    twitter: "https://twitter.com/carllippert",
    github: "https://github.com/tryanything-ai/anything",
  },
};
