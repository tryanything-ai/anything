import "./globals.css";
import "ui/styles.css";

import type { Metadata } from "next";
import { ReactNode, Suspense, JSX } from "react";
import { siteConfig } from "../config/site";
import { dm_sans, inter } from "../lib/fonts";
import { PHProvider, PostHogPageview } from "./providers";

export const metadata: Metadata = {
  title: {
    default: siteConfig.name,
    template: `%s | ${siteConfig.name}`,
  },
  description: siteConfig.description,
  keywords: ["Automation", "AI", "Zapier", "Node Red", "N8N"],
  authors: [
    {
      name: "anything",
      url: "https://tryanything.xyz",
    },
  ],
  creator: "anything",
  metadataBase: new URL(siteConfig.url),
  alternates: {
    canonical: "/",
  },
  openGraph: {
    type: "website",
    locale: "en_US",
    title: siteConfig.name,
    description: siteConfig.description,
    siteName: siteConfig.name,
  },
  twitter: {
    card: "summary_large_image",
    title: siteConfig.name,
    description: siteConfig.description,
    images: ["/og.jpg"],
    creator: "@carllippert",
  },
  icons: {
    icon: "/favicons/favicon.ico",
    shortcut: "/favicons/favicon-16x16.png",
    apple: "/favicons/apple-touch-icon.png",
  },
  manifest: `${siteConfig.url}/favicons/site.webmanifest`,
};

export default async function RootLayout({
  children,
}: {
  children: ReactNode;
}): Promise<JSX.Element> {
  return (
    <html
      // data-theme="light"
      className={`${inter.variable} ${dm_sans.variable}`}
      lang="en"
      suppressHydrationWarning
    >
      <head />
      <Suspense>
        <PostHogPageview />
      </Suspense>
      <PHProvider>
        <body data-theme="dark" className="text-slate-12 font-sans">{children}</body>
      </PHProvider>
    </html>
  );
}
