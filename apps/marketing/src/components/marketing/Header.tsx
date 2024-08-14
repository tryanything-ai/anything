"use client";

import { Dialog } from "@headlessui/react";
import Link from "next/link";
import { useState } from "react";
import { VscClose, VscMenu } from "react-icons/vsc";
import { FaDiscord } from "react-icons/fa";
import { Stargazer } from "@repo/ui/components/ui/Stargazer";

export function Header({ stargazers_count }: { stargazers_count: number }) {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  const handleLinkClick = () => {
    setMobileMenuOpen(false);
  };

  return (
    <header className="text-white border-b border-slate-6 bg-slate-1/5 py-3 backdrop-blur-lg">
      <nav
        className="mx-auto flex max-w-7xl items-center justify-between px-6 lg:px-8"
        aria-label="Global"
      >
        <div className="flex items-center gap-4 lg:flex-1">
          <Link href="/" className="-m-1.5 p-1.5" onClick={handleLinkClick}>
            <span className="sr-only">Anything</span>
            <div className="flex gap-2">
              <span className="body-semibold">Anything</span>
            </div>
          </Link>
          <Stargazer count={stargazers_count} />

          <Link
            href="/platform"
            className="-m-1.5 p-1.5 lg:flex hidden"
            onClick={handleLinkClick}
          >
            <span className="sr-only">Platform</span>
            <div className="flex gap-2 ml-4">
              <span className="">Platform</span>
            </div>
          </Link>
        </div>

        {/* Ensure Discord button is always on the right side */}
        <div className="lg:flex items-center hidden">
          <a
            href="https://discord.gg/VRBKaqjprE"
            className="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-slate-11"
            role="button" // Add role
            tabIndex={0} // Make it focusable
            onKeyDown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                window.location.href = "https://discord.gg/VRBKaqjprE";
              }
            }} // Handle keyboard events
          >
            <span className="sr-only">Discord</span>
            <FaDiscord className="h-6 w-6" aria-hidden="true" />
          </a>
        </div>
        {/* Ensure Discord button is always on the right side */}
        <div
          className="lg:hidden"
          role="button" // Add role
          tabIndex={0} // Make it focusable
          onClick={() => setMobileMenuOpen(true)}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              setMobileMenuOpen(true);
            }
          }} // Handle keyboard events
        >
          <VscMenu className="h-6 w-6" aria-hidden="true" />
        </div>
      </nav>

      {/* Mobile Menu Dialog */}
      <Dialog
        as="div"
        className="lg:hidden"
        open={mobileMenuOpen}
        onClose={setMobileMenuOpen}
      >
        <div className="text-white fixed inset-0 z-10" />
        <Dialog.Panel className="text-white fixed inset-y-0 right-0 z-10 w-full overflow-y-auto bg-slate-1 p-6 sm:max-w-sm sm:ring-1 sm:ring-slate-6">
          <div className="flex items-center justify-between">
            <Link href="/" className="-m-1.5 p-1.5" onClick={handleLinkClick}>
              <span className="sr-only">Anything</span>
              <div className="flex gap-2">
                <span className="body-semibold">Anything</span>
              </div>
            </Link>
            <button
              type="button"
              className="-m-2.5 rounded-md p-2.5 text-slate-11"
              onClick={() => setMobileMenuOpen(false)}
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                  setMobileMenuOpen(false);
                }
              }} // Handle keyboard events
            >
              <span className="sr-only">Close menu</span>
              <VscClose className="h-6 w-6" aria-hidden="true" />
            </button>
          </div>
          <div className="flex items-center justify-between mt-10">
            <Link
              href="/platform"
              className="-m-1.5 p-1.5"
              onClick={handleLinkClick}
            >
              <span className="sr-only">Platform</span>
              <div className="flex gap-2">
                <span className="body-semibold">Platform</span>
              </div>
            </Link>
          </div>
          <div className="flex items-center justify-between mt-10">
            <Link
              href="/local"
              className="-m-1.5 p-1.5"
              onClick={handleLinkClick}
            >
              <span className="sr-only">Local AI (legacy)</span>
              <div className="flex gap-2">
                <span className="body-semibold">Local AI (legacy)</span>
              </div>
            </Link>
          </div>
          <div className="flex items-center justify-between mt-10">
            <a
              href="https://discord.gg/VRBKaqjprE"
              className="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-slate-11"
              role="button" // Add role
              tabIndex={0} // Make it focusable
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                  window.location.href = "https://discord.gg/VRBKaqjprE";
                }
              }} // Handle keyboard events
            >
              <span className="sr-only">Discord</span>
              <FaDiscord className="h-6 w-6" aria-hidden="true" />
            </a>
          </div>
        </Dialog.Panel>
      </Dialog>
    </header>
  );
}
