type SiteConfig = {
  name: string;
  description: string;
  url: string;
  ogImage: string;
  links: {
    twitter: string;
    github: string;
  };
};

export const siteConfig: SiteConfig = {
  name: "Anything AI",
  description: "The easiest way to automate your business",
  url: `${process.env.NEXT_PUBLIC_HOSTED_URL}`,
  ogImage: `${process.env.NEXT_PUBLIC_HOSTED_URL}/og.jpg`,
  links: {
    twitter: "https://x.com/_anything_ai",
    github: "https://github.com/tryanything-ai/anything",
  },
};
