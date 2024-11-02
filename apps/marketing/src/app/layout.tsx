import { Inter as FontSans } from "next/font/google";
import "@repo/ui/globals.css";

import type { Metadata } from "next";
import { ReactNode, Suspense, JSX } from "react";
import { siteConfig } from "../config/site";
import { cn } from "@/lib/utils";
import { PHProvider, PostHogPageview } from "./providers";

const fontSans = FontSans({
  subsets: ["latin"],
  variable: "--font-sans",
});

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
  // alternates: {
  //   canonical: "https://tryanything.xyz",
  // },
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
    creator: "@_anything_ai",
  },
  icons: {
    icon: "/favicons/favicon.ico",
    shortcut: "/favicons/favicon-16x16.png",
    apple: "/favicons/apple-touch-icon.png",
  },
};

export default function RootLayout({
  children,
}: {
  children: ReactNode;
}): JSX.Element {
  return (
    <html
      lang="en"
      className={cn(
        "min-h-screen bg-background font-sans antialiased",
        fontSans.variable,
      )}
      suppressHydrationWarning
    >
      <Suspense>
        <PostHogPageview />
      </Suspense>
      <body className="bg-background text-foreground">
        <PHProvider>
          <main className="min-h-screen">{children}</main>
        </PHProvider>
      </body>
    </html>
  );
}
