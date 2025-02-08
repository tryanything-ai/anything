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
import { MARKETING_EVENTS } from "@/app/posthog";

export function Header({ stargazers_count }: { stargazers_count: number }) {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const posthog = usePostHog();

  const handleLinkClick = (eventName: string) => {
    setMobileMenuOpen(false);
    posthog.capture(eventName);
  };

  const handleLogin = () => {
    posthog.capture(MARKETING_EVENTS.LOGIN_CLICK);
    window.location.href = `https://app.${window.location.hostname.replace('www.', '')}/login`;
  };

  const handleSignup = () => {
    posthog.capture(MARKETING_EVENTS.SIGNUP_CLICK);
    window.location.href = `https://app.${window.location.hostname.replace('www.', '')}/signup`;
  };

  const handleGithub = () => {
    posthog.capture(MARKETING_EVENTS.GITHUB_CLICK);
    window.location.href = "https://github.com/tryanything-ai/anything";
  };

  return (
    <header className="text-slate-900 border-b border-slate-200 bg-white py-3 font-sans">
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
          <Link href="/" className="-m-1.5 p-1.5" onClick={() => handleLinkClick(MARKETING_EVENTS.HOME_VIEW)}>
            <span className="sr-only">Anything AI</span>
            <div className="flex gap-2 items-center">
              <span className="font-bold tracking-tight text-xl">
                Anything AI
              </span>
              <span className="text-xs bg-pink-100 text-pink-800 px-2 py-1 rounded-full hidden sm:inline-block">
                beta
              </span>
            </div>
          </Link>
          <div className="hidden sm:block">
            <Stargazer count={stargazers_count} />
          </div>

          {/* <Link
            href="/templates/workflows"
            className="-m-1.5 p-1.5 lg:flex hidden"
            onClick={() => handleLinkClick(MARKETING_EVENTS.TEMPLATE_VIEW)}
          >
            <span className="sr-only">Templates</span>
            <div className="flex gap-2 ml-4">
              <span className="text-base font-medium">Templates</span>
            </div>
          </Link>
          <Link
            href="/templates/actions"
            className="-m-1.5 p-1.5 lg:flex hidden"
            onClick={() => handleLinkClick(MARKETING_EVENTS.INTEGRATION_VIEW)}
          >
            <span className="sr-only">Integrations</span>
            <div className="flex gap-2 ml-4">
              <span className="text-base font-medium">Integrations</span>
            </div>
          </Link> */}
        </div>

        <div className="flex items-center ml-auto">
          <Button
            className="h-11 mr-2 rounded-full hidden sm:inline-flex"
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
            <Link href="/" className="-m-1.5 p-1.5" onClick={() => handleLinkClick(MARKETING_EVENTS.HOME_VIEW)}>
              <span className="sr-only">Anything</span>
              <div className="flex gap-2">
                <span className="text-xl font-bold tracking-tight text-slate-900">
                  Anything
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
          {/* <div className="flex items-center justify-between mt-10">
            <Link
              href="/templates/workflows"
              className="-m-1.5 p-1.5 text-slate-900 hover:text-slate-600 transition-colors duration-200"
              onClick={() => handleLinkClick(MARKETING_EVENTS.TEMPLATE_VIEW)}
            >
              <span className="sr-only">Templates</span>
              <div className="flex gap-2">
                <span className="text-base font-medium">Templates</span>
              </div>
            </Link>
          </div>
          <div className="flex items-center justify-between mt-10">
            <Link
              href="/templates/actions"
              className="-m-1.5 p-1.5 text-slate-900 hover:text-slate-600 transition-colors duration-200"
              onClick={() => handleLinkClick(MARKETING_EVENTS.INTEGRATION_VIEW)}
            >
              <span className="sr-only">Integrations</span>
              <div className="flex gap-2">
                <span className="text-base font-medium">Integrations</span>
              </div>
            </Link>
          </div> */}

   <div className="flex items-center justify-between mt-10">
            <a
              href="https://github.com/tryanything-ai/anything"
              className="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-slate-600 hover:text-slate-900 transition-colors duration-200"
              role="button"
              tabIndex={0}
              onClick={handleGithub}
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                  handleGithub();
                }
              }}
            >
              <span className="sr-only">GitHub</span>
              <FaGithub className="h-6 w-6" aria-hidden="true" />
            </a>
          </div>
          {/* <div className="flex items-center justify-between mt-10">
            <a
              href="https://discord.gg/VRBKaqjprE"
              className="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-slate-600 hover:text-slate-900 transition-colors duration-200"
              role="button"
              tabIndex={0}
              onClick={handleDiscord}
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                  handleDiscord();
                }
              }}
            >
              <span className="sr-only">Discord</span>
              <FaDiscord className="h-6 w-6" aria-hidden="true" />
            </a>
          </div> */}
        </Dialog.Panel>
      </Dialog>
    </header>
  );
}
