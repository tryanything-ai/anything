"use client";

import { Dialog } from "@headlessui/react";
import Link from "next/link";
import { useState } from "react";
import { VscClose, VscMenu } from "react-icons/vsc";
import { FaGithub } from "react-icons/fa";
import { Stargazer } from "@/components/ui/Stargazer";
import ShimmerButton from "@repo/ui/components/magicui/shimmer-button";
import { Button } from "@repo/ui/components/ui/button";
import { usePostHog } from "posthog-js/react";
import { MARKETING_EVENTS } from "../../posthog/events";

export function Header({
  stargazers_count,
}: {
  stargazers_count: number | null;
}) {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const posthog = usePostHog();

  const handleLinkClick = (eventName: string) => {
    setMobileMenuOpen(false);
    posthog.capture(eventName);
  };

  const handleLogin = async () => {
    await posthog.capture(MARKETING_EVENTS.LOGIN_CLICK);
    window.location.href = `https://app.${window.location.hostname.replace("www.", "")}/login`;
  };

  const handleSignup = async () => {
    await posthog.capture(MARKETING_EVENTS.SIGNUP_CLICK);
    window.location.href = `https://app.${window.location.hostname.replace("www.", "")}/signup`;
  };

  const handleGithub = async () => {
    await posthog.capture(MARKETING_EVENTS.GITHUB_CLICK);
    window.location.href = "https://github.com/tryanything-ai/anything";
  };

  return (
    <header className="text-slate-900 border-b border-slate-200 bg-white py-4 font-sans sticky top-0 z-50 shadow-sm">
      <nav
        className="mx-auto flex max-w-7xl items-center justify-between px-6 lg:px-8"
        aria-label="Global"
      >
        <div className="flex items-center gap-4">
          <div
            className="lg:hidden mr-2"
            role="button"
            tabIndex={0}
            onClick={() => setMobileMenuOpen(true)}
            onKeyDown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                setMobileMenuOpen(true);
              }
            }}
          >
            <VscMenu className="h-6 w-6 text-slate-900" aria-hidden="true" />
          </div>
          <Link
            href="/"
            className="-m-1.5 p-1.5"
            onClick={() => handleLinkClick(MARKETING_EVENTS.HOME_VIEW)}
          >
            <span className="sr-only">Anything AI</span>
            <div className="flex gap-2 items-center">
              <span className="font-bold tracking-tight text-xl">
                Anything AI
              </span>
            </div>
          </Link>
          <div className="hidden sm:block">
            {stargazers_count !== null && (
              <Stargazer count={stargazers_count} />
            )}
          </div>

          {/* <div className="hidden lg:flex space-x-6 ml-6">
            <Link
              href="/features"
              className="text-base font-medium text-slate-700 hover:text-purple-600 transition-colors"
              onClick={() => handleLinkClick(MARKETING_EVENTS.FEATURE_VIEW)}
            >
              Features
            </Link>
            <Link
              href="/pricing"
              className="text-base font-medium text-slate-700 hover:text-purple-600 transition-colors"
              onClick={() => handleLinkClick(MARKETING_EVENTS.PRICING_VIEW)}
            >
              Pricing
            </Link>
            <Link
              href="/about"
              className="text-base font-medium text-slate-700 hover:text-purple-600 transition-colors"
              onClick={() => handleLinkClick(MARKETING_EVENTS.ABOUT_VIEW)}
            >
              About
            </Link>
          </div> */}
        </div>

        <div className="flex items-center ml-auto">
          <Button
            className="h-11 mr-3 rounded-full hidden sm:inline-flex hover:bg-purple-50 hover:text-purple-600 transition-colors"
            variant="outline"
            onClick={handleLogin}
          >
            Login
          </Button>
          <ShimmerButton
            background="rgb(147 51 234)"
            className="p-2 font-bold"
            onClick={handleSignup}
          >
            Get Started
          </ShimmerButton>
        </div>
      </nav>

      {/* Mobile Menu Dialog */}
      <Dialog
        as="div"
        className="lg:hidden"
        open={mobileMenuOpen}
        onClose={setMobileMenuOpen}
      >
        <div className="fixed inset-0 z-10 bg-black/30" />
        <Dialog.Panel className="fixed inset-y-0 right-0 z-10 w-full overflow-y-auto bg-white p-6 sm:max-w-sm sm:ring-1 sm:ring-slate-900/10">
          <div className="flex items-center justify-between">
            <Link
              href="/"
              className="-m-1.5 p-1.5"
              onClick={() => handleLinkClick(MARKETING_EVENTS.HOME_VIEW)}
            >
              <span className="sr-only">Anything AI</span>
              <div className="flex gap-2">
                <span className="text-xl font-bold tracking-tight text-slate-900">
                  Anything AI
                </span>
              </div>
            </Link>
            <button
              type="button"
              className="-m-2.5 rounded-md p-2.5 text-slate-600 hover:text-slate-900 transition-colors duration-200"
              onClick={() => setMobileMenuOpen(false)}
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                  setMobileMenuOpen(false);
                }
              }}
            >
              <span className="sr-only">Close menu</span>
              <VscClose className="h-6 w-6" aria-hidden="true" />
            </button>
          </div>

          {/* <div className="mt-8 space-y-6">
            <Link
              href="/features"
              className="block text-base font-medium text-slate-900 hover:text-purple-600 transition-colors"
              onClick={() => handleLinkClick(MARKETING_EVENTS.FEATURE_VIEW)}
            >
              Features
            </Link>
            <Link
              href="/pricing"
              className="block text-base font-medium text-slate-900 hover:text-purple-600 transition-colors"
              onClick={() => handleLinkClick(MARKETING_EVENTS.PRICING_VIEW)}
            >
              Pricing
            </Link>
            <Link
              href="/about"
              className="block text-base font-medium text-slate-900 hover:text-purple-600 transition-colors"
              onClick={() => handleLinkClick(MARKETING_EVENTS.ABOUT_VIEW)}
            >
              About
            </Link>
          </div> */}

          <div className="mt-8 flex flex-col space-y-4">
            <Button
              className="w-full rounded-full justify-center"
              variant="outline"
              onClick={handleLogin}
            >
              Login
            </Button>
            <Button
              className="w-full rounded-full justify-center bg-purple-600 hover:bg-purple-700 text-white"
              onClick={handleSignup}
            >
              Get Started
            </Button>
          </div>

          <div className="mt-8 pt-6 border-t border-slate-200">
            <a
              href="https://github.com/tryanything-ai/anything"
              className="flex items-center text-slate-700 hover:text-slate-900 transition-colors"
              onClick={handleGithub}
            >
              <FaGithub className="h-5 w-5 mr-2" />
              <span>GitHub</span>
            </a>
          </div>
        </Dialog.Panel>
      </Dialog>
    </header>
  );
}
