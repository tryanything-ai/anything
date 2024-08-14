"use client";

import type { VariantProps } from "class-variance-authority";
import type { ReactNode } from "react";

import { Button, buttonVariants } from "@repo/ui/components/ui/button";

export function SignUpButton({
  type,
  children,
}: {
  type: VariantProps<typeof buttonVariants>["variant"];
  children: any;
}) {
  async function handleGithub() {
    // signIn("github", { callbackUrl: "/billing" });
  }

  return (
    <Button
      type="button"
      variant={type}
      className="w-[256px]"
      onClick={() => {
        handleGithub();
      }}
    >
      {children}
    </Button>
  );
}
