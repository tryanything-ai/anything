import "./globals.css";
import "ui/styles.css";

import type { Metadata } from "next";
import { ReactNode, Suspense } from "react";
import { siteConfig } from "../config/site";
import { dm_sans, inter } from "../lib/fonts";
import { PHProvider, PostHogPageview } from "./providers";
// import { Toaster } from "@/components/ui/toaster";

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
}) {
  return (
    <html
      lang="en"
      className={`${inter.variable} ${dm_sans.variable}`}
      suppressHydrationWarning
    >
      <head />
      {/* Body */}
      <Suspense>
        <PostHogPageview />
      </Suspense>
      <PHProvider>
        <body className=" text-slate-12 font-sans">{children}</body>
      </PHProvider>
      {/* <body className="font-sans text-slate-12">
        {children}
        <Toaster />
      </body> */}
    </html>
  );
}
