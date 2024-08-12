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
  const res = await fetch(
    "https://api.github.com/repos/tryanything-ai/anything",
    {
      method: "GET",
      next: { revalidate: 60 },
    }
  );
  const data = await res.json();

  const stargazers_count: number = data.stargazers_count;

  return (
    <div>
      <Header stargazers_count={stargazers_count} />
      <main>{children}</main>
      <Footer />
    </div>
  );
}
