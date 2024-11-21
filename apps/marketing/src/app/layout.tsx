import { Inter as FontSans } from "next/font/google";
import "@repo/ui/globals.css";
import Script from "next/script";
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
      url: "https://www.tryanything.xyz",
    },
  ],
  creator: "anything",
  metadataBase: new URL(siteConfig.url),
  alternates: {
    canonical: "https://www.tryanything.xyz",
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
      {/* RB2B Script */}
      {/* <Script id="reb2b-script" strategy="afterInteractive" dangerouslySetInnerHTML={{ __html: `!function () {var reb2b = window.reb2b = window.reb2b || []; if (reb2b.invoked) return;reb2b.invoked = true;reb2b.methods = ["identify", "collect"]; reb2b.factory = function (method) {return function () {var args = Array.prototype.slice.call(arguments); args.unshift(method);reb2b.push(args);return reb2b;};}; for (var i = 0; i < reb2b.methods.length; i++) {var key = reb2b.methods[i];reb2b[key] = reb2b.factory(key);} reb2b.load = function (key) {var script = document.createElement("script");script.type = "text/javascript";script.async = true; script.src = "https://s3-us-west-2.amazonaws.com/b2bjsstore/b/" + key + "/9NMMZHPY38NW.js.gz"; var first = document.getElementsByTagName("script")[0]; first.parentNode.insertBefore(script, first);}; reb2b.SNIPPET_VERSION = "1.0.1";reb2b.load("9NMMZHPY38NW");}();` }} /> */}
      <body className="bg-background text-foreground">
        <PHProvider>
          <main className="min-h-screen">{children}</main>
        </PHProvider>
      </body>
    </html>
  );
}
