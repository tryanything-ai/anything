"use client";

import type { VariantProps } from "class-variance-authority";
import type { ReactNode } from "react";

import { Button, buttonVariants } from "@/components/ui/Button";

export function SignUpButton({
  type,
  children,
}: {
  type: VariantProps<typeof buttonVariants>["variant"];
  children: ReactNode;
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
