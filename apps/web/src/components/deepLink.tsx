"use client";

import React, { FC, MouseEvent } from "react";

interface DeepLinkProps {
  href: string;
  fallbackUrl?: string;
  children: React.ReactNode;
}

const DeepLink: FC<DeepLinkProps> = ({ href, fallbackUrl, children }) => {
  //   if (typeof window === "undefined") {
  //     console.log("returning normal link");
  //     return <a href={href}>{children}</a>;
  //   }

  const handleDeepLinkClick = (e: MouseEvent<HTMLAnchorElement>): void => {
    e.preventDefault();

    // Try opening the deep link
    const opened = window.open(href);

    if (!opened) {
      if (fallbackUrl) {
        // If fallbackUrl is provided, navigate t o it
        window.location.href = fallbackUrl;
      } else {
        // Otherwise, show an alert
        alert(
          "Failed to open the link. Please ensure you have the required app installed."
        );
      }
    }
  };

  return (
    <a href={href} onClick={handleDeepLinkClick}>
      {children}
    </a>
  );
};

export default DeepLink;
