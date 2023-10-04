"use client";

// import { signIn } from "next-auth/react";

// import { Button } from "@/components/ui/Button";

export function SignInButton({ className }: { className: string }) {
  async function handleGithub() {
    // signIn("github", { callbackUrl: "/generate" });
  }

  return (
    <div className="flex flex-col items-center gap-2 md:flex-row md:gap-4">
      <button
        type="button"
        // variant="text"
        className={className + "btn"}
        onClick={() => {
          handleGithub();
        }}
      >
        Login
      </button>
    </div>
  );
}
