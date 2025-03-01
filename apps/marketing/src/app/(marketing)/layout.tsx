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
  let stargazers_count: number | null = null;

  try {
    // Fetch GitHub stars with a daily revalidation
    const res = await fetch(
      "https://api.github.com/repos/tryanything-ai/anything",
      {
        method: "GET",
        // Cache for 24 hours (86400 seconds)
        next: { revalidate: 86400 },
        headers: {
          "User-Agent": "Anything-Marketing-Website",
        },
      },
    );

    // Check if the response was successful
    if (!res.ok) {
      const errorData = await res.json().catch(() => ({}));
      console.error(`GitHub API error (${res.status}):`, errorData);
      throw new Error(`GitHub API returned ${res.status}`);
    }

    const data = await res.json();

    if (data && typeof data.stargazers_count === "number") {
      stargazers_count = data.stargazers_count;
    }
  } catch (error) {
    console.error("Failed to fetch stargazers count:", error);
    // If we have an error, we'll pass null to the Header component
  }

  return (
    <div>
      <Header stargazers_count={stargazers_count} />
      <main>{children}</main>
      <Footer />
    </div>
  );
}
