import type { Metadata } from "next";
import React from "react";

import { Footer } from "@/components/Footer";
import { Header } from "@/components/marketing/Header";

interface MarketingLayoutProps {
  children: React.ReactNode;
}

export const metadata: Metadata = {
  // title: "Anything",
  // description: "Automate",
};

export default async function MarketingLayout({
  children,
}: MarketingLayoutProps) {
  let stargazers_count: number = 0;

  try {
    const res = await fetch(
      "https://api.github.com/repos/tryanything-ai/anything",
      {
        method: "GET",
        next: { revalidate: 60 },
      }
    );
    const data = await res.json();

    if (data && typeof data.stargazers_count === 'number') {
      stargazers_count = data.stargazers_count;
    }
  } catch (error) {
    console.error("Failed to fetch stargazers count:", error);
  }

  return (
    <div>
      <Header stargazers_count={stargazers_count} />
      <main>{children}</main>
      <Footer />
    </div>
  );
}
