"use client";

import Link from "next/link";
import { useState, useEffect } from "react";
import { useRouter, useSearchParams, usePathname } from "next/navigation";
import { createClient } from "@/lib/supabase/client";
import { usePostHog } from "posthog-js/react";
import { AUTH_EVENTS } from "@/posthog/events";

import { Button } from "@repo/ui/components/ui/button";
import { Input } from "@repo/ui/components/ui/input";
import { Label } from "@repo/ui/components/ui/label";
import OrbitingCirclesIntegrations from "@repo/ui/components/magicui/orbiting-circles-integrations";

const AuthPage = () => {
  const [isLogin, setIsLogin] = useState(true);
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const router = useRouter();
  const searchParams = useSearchParams();
  const pathname = usePathname();
  const returnUrl = searchParams.get("returnUrl");
  const posthog = usePostHog();

  useEffect(() => {
    setIsLogin(pathname === "/login");
  }, [pathname]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setMessage("");
    const supabase = await createClient();

    if (isLogin) {
      posthog.capture(AUTH_EVENTS.LOGIN_ATTEMPT, { email });
      const { error } = await supabase.auth.signInWithPassword({
        email,
        password,
      });

      if (error) {
        setIsLoading(false);
        setMessage("Could not authenticate user");
        posthog.capture(AUTH_EVENTS.LOGIN_ERROR, { error: error.message });
      } else {
        posthog.capture(AUTH_EVENTS.LOGIN_SUCCESS, { email });
        router.push(returnUrl || "/");
      }
    } else {
      posthog.capture(AUTH_EVENTS.SIGNUP_ATTEMPT, { email });
      const { error } = await supabase.auth.signUp({
        email,
        password,
        options: {
          emailRedirectTo: `${window.location.origin}/auth/callback?returnUrl=${returnUrl || ""}`,
        },
      });

      if (error) {
        setIsLoading(false);
        setMessage("Could not create user");
        posthog.capture(AUTH_EVENTS.SIGNUP_ERROR, { error: error.message });
      } else {
        setIsLoading(false);
        setMessage("Check email to continue sign in process");
        posthog.capture(AUTH_EVENTS.EMAIL_VERIFICATION_SENT, { email });
      }
    }
   
  };

  return (
    <div className="w-full lg:grid lg:min-h-[600px] lg:grid-cols-2 xl:min-h-[800px]">
      <div className="flex items-center justify-center py-12">
        <div className="mx-auto grid w-[350px] gap-6">
          <div className="grid gap-2 text-center">
            <h1 className="text-3xl font-bold">
              {isLogin ? "Login" : "Sign Up"}
            </h1>
            <p className="text-balance text-muted-foreground">
              {isLogin
                ? "Enter your email below to login to your account"
                : "Create an account to get started"}
            </p>
          </div>
          <form onSubmit={handleSubmit} className="grid gap-4">
            <div className="grid gap-2">
              <Label htmlFor="email">Email</Label>
              <Input
                id="email"
                type="email"
                placeholder="m@example.com"
                required
                value={email}
                onChange={(e) => setEmail(e.target.value)}
              />
            </div>
            <div className="grid gap-2">
              <div className="flex items-center">
                <Label htmlFor="password">Password</Label>
              </div>
              <Input
                id="password"
                type="password"
                required
                value={password}
                onChange={(e) => setPassword(e.target.value)}
              />
            </div>
            <Button disabled={isLoading} type="submit" className="w-full">
              {isLoading ? (
                <div className="flex items-center gap-2">
                  <div className="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent" />
                  {isLogin ? "Logging in..." : "Signing up..."}
                </div>
              ) : (
                <>{isLogin ? "Login" : "Sign Up"}</>
              )}
            </Button>
          </form>
          <div className="mt-4 text-center text-sm">
            {isLogin ? "Don't have an account? " : "Already have an account? "}
            <Link href={isLogin ? "/signup" : "/login"} className="underline">
              {isLogin ? "Sign up" : "Login"}
            </Link>
          </div>
          {message && (
            <p className="mt-4 p-4 bg-foreground/10 text-foreground text-center">
              {message}
            </p>
          )}
        </div>
      </div>
      <div className="hidden bg-muted lg:block">
        {/* <Image
          src="/placeholder.svg"
          alt="Image"
          width="1920"
          height="1080"
          className="h-full w-full object-cover dark:brightness-[0.2] dark:grayscale"
        /> */}
        <OrbitingCirclesIntegrations />
      </div>
    </div>
  );
};

export default AuthPage;
