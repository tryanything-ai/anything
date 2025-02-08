import { headers } from "next/headers";
import { Button } from "@repo/ui/components/ui/button";
import Llama from "../../../public/llamascreenshot.png";
import Features from "@/components/LocalFeatures";
import Faq from "@/components/LocalFaq";
import PricingGroup from "@/components/marketing/pricing/pricing-group";
import Image from "next/image";

import HeroVideoDialog from "@repo/ui/components/magicui/hero-video-dialog";

export default function IndexPage() {
  const headerList = headers();

  return (
    <>
      <div className="mt-16 flex flex-col items-center gap-4">
        <h1 className="text-5xl md:text-7xl font-bold text-center max-w-4xl px-4 md:px-0">
          Automate <span className="text-purple-600">Anything</span> with AI
        </h1>
        <p className="text-xl md:text-2xl text-center text-gray-600 max-w-2xl px-4 md:px-0 mt-4">
          Easily build AI automations for your business.
        </p>
      </div>
      <div className="max-w-7xl mx-auto px-5 mt-10">
        <HeroVideoDialog
          className="dark:hidden block"
          animationStyle="from-center"
          videoSrc="https://www.youtube.com/embed/qh3NGpYRG3I?si=4rb-zSdDkVK9qxxb"
          thumbnailSrc="/anything_screenshot.png"
          thumbnailAlt="Anything Screenshot"
        />
      </div>

      <div className="overflow-hidden bg-white py-12 sm:py-32">
        <div className="mx-auto max-w-7xl px-6 lg:px-8">
          <div className="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-8 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
            <div className="lg:pr-8 lg:pt-4">
              <div className="lg:max-w-lg">
                <h2 className="text-base font-semibold leading-7 text-purple-600">
                  Build, Test, Deploy
                </h2>
                <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
                  From idea to production in minutes
                </p>
                <p className="mt-6 text-lg leading-8 text-gray-600">
                  Build powerful workflows, test them thoroughly, and deploy
                  with confidence. Our end-to-end platform makes it easy to go
                  from concept to production-ready automation in record time.
                </p>
                <dl className="mt-10 max-w-xl space-y-8 text-base leading-7 text-gray-600 lg:max-w-none">
                  <div className="relative pl-9">
                    <dt className="inline font-semibold text-gray-900">
                      <svg
                        className="absolute left-1 top-1 h-5 w-5 text-purple-600"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                      >
                        <path
                          fillRule="evenodd"
                          d="M10 2a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5A.75.75 0 0110 2zM10 15a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5A.75.75 0 0110 15zM10 7a3 3 0 100 6 3 3 0 000-6z"
                          clipRule="evenodd"
                        />
                      </svg>
                      Visual Builder
                    </dt>
                    <dd className="inline">
                      {" "}
                      Build workflows visually with our intuitive drag-and-drop interface. No coding required.
                    </dd>
                  </div>
                  <div className="relative pl-9">
                    <dt className="inline font-semibold text-gray-900">
                      <svg
                        className="absolute left-1 top-1 h-5 w-5 text-purple-600"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                      >
                        <path d="M3.196 12.87l-.825.483a.75.75 0 000 1.294l7.25 4.25a.75.75 0 00.758 0l7.25-4.25a.75.75 0 000-1.294l-.825-.484-5.666 3.322a2.25 2.25 0 01-2.276 0L3.196 12.87z" />
                        <path d="M10.38 1.103a.75.75 0 00-.76 0l-7.25 4.25a.75.75 0 000 1.294l7.25 4.25a.75.75 0 00.76 0l7.25-4.25a.75.75 0 000-1.294l-7.25-4.25z" />
                      </svg>
                      One-Click Deploy
                    </dt>
                    <dd className="inline">
                      {" "}
                      Deploy tested workflows to production instantly with a single click.
                    </dd>
                  </div>
                </dl>
              </div>
            </div>
            <div>
              <Image
                src="/images/test_and_deploy.png"
                alt="Screenshot showing workflow building and testing interface"
                className="w-full max-w-[35.2rem] mx-auto rounded-xl shadow-xl ring-1 ring-gray-400/10 sm:w-[52.8rem]"
                width={880}
                height={660}
              />
            </div>
          </div>
        </div>
      </div>
      <div className="overflow-hidden bg-white py-12 sm:py-32">
        <div className="mx-auto max-w-7xl px-6 lg:px-8">
          <div className="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-8 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
            <div className="order-2 lg:order-1">
              <Image
                src="/images/reuse.png"
                alt="Screenshot showing reusable configured components"
                className="w-full max-w-[35.2rem] mx-auto rounded-xl shadow-xl ring-1 ring-gray-400/10 sm:w-[52.8rem]"
                width={880}
                height={660}
              />
            </div>
            <div className="lg:pr-8 lg:pt-4 order-1 lg:order-2">
              <div className="lg:max-w-lg">
                <h2 className="text-base font-semibold leading-7 text-purple-600">
                  Reusable Components
                </h2>
                <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
                  Configure once, use everywhere
                </p>
                <p className="mt-6 text-lg leading-8 text-gray-600">
                  Create configured components that can be reused across all your workflows. Save time by setting up integrations once and reusing them wherever needed.
                </p>
                <dl className="mt-10 max-w-xl space-y-8 text-base leading-7 text-gray-600 lg:max-w-none">
                  <div className="relative pl-9">
                    <dt className="inline font-semibold text-gray-900">
                      <svg
                        className="absolute left-1 top-1 h-5 w-5 text-purple-600"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                      >
                        <path
                          fillRule="evenodd"
                          d="M9.664 1.319a.75.75 0 01.672 0 41.059 41.059 0 018.198 5.424.75.75 0 01-.254 1.285 31.372 31.372 0 00-7.86 3.83.75.75 0 01-.84 0 31.508 31.508 0 00-2.08-1.287V9.394c0-.244.116-.463.302-.592a35.504 35.504 0 013.305-2.033.75.75 0 00-.714-1.319 37 37 0 00-3.446 2.12A2.216 2.216 0 006 9.393v.38a31.293 31.293 0 00-4.28-1.746.75.75 0 01-.254-1.285 41.059 41.059 0 018.198-5.424zM6 11.459a29.848 29.848 0 00-2.455-1.158 41.029 41.029 0 00-.39 3.114.75.75 0 00.419.74c.528.256 1.046.53 1.554.82-.21.324-.455.63-.739.914a.75.75 0 101.06 1.06c.37-.369.69-.77.96-1.193a26.61 26.61 0 013.095 2.348.75.75 0 00.992 0 26.547 26.547 0 015.93-3.95.75.75 0 00.42-.739 41.053 41.053 0 00-.39-3.114 29.925 29.925 0 00-5.199 2.801 2.25 2.25 0 01-2.514 0c-.41-.275-.826-.541-1.25-.797a6.985 6.985 0 01-1.084 3.45 26.503 26.503 0 00-1.281-.78A5.487 5.487 0 006 12v-.54z"
                          clipRule="evenodd"
                        />
                      </svg>
                      Consistent Configuration
                    </dt>
                    <dd className="inline">
                      {" "}
                      Configure components once and maintain consistent settings across all workflows.
                    </dd>
                  </div>
                </dl>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="overflow-hidden bg-white py-12 sm:py-32">
        <div className="mx-auto max-w-7xl px-6 lg:px-8">
          <div className="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-8 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
            <div className="lg:pr-8 lg:pt-4 order-1">
              <div className="lg:max-w-lg">
                <h2 className="text-base font-semibold leading-7 text-purple-600">
                  Team Collaboration
                </h2>
                <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
                  Share and collaborate
                </p>
                <p className="mt-6 text-lg leading-8 text-gray-600">
                  Share your actions and workflow templates with your team. Collaborate on automation projects and maintain consistency across your organization.
                </p>
                <dl className="mt-10 max-w-xl space-y-8 text-base leading-7 text-gray-600 lg:max-w-none">
                  <div className="relative pl-9">
                    <dt className="inline font-semibold text-gray-900">
                      <svg
                        className="absolute left-1 top-1 h-5 w-5 text-purple-600"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                      >
                        <path d="M10 9a3 3 0 100-6 3 3 0 000 6zM6 8a2 2 0 11-4 0 2 2 0 014 0zM1.49 15.326a.78.78 0 01-.358-.442 3 3 0 014.308-3.516 6.484 6.484 0 00-1.905 3.959c-.023.222-.014.442.025.654a4.97 4.97 0 01-2.07-.655zM16.44 15.98a4.97 4.97 0 002.07-.654.78.78 0 00.357-.442 3 3 0 00-4.308-3.517 6.484 6.484 0 011.907 3.96 2.32 2.32 0 01-.026.654zM18 8a2 2 0 11-4 0 2 2 0 014 0zM5.304 16.19a.844.844 0 01-.277-.71 5 5 0 019.947 0 .843.843 0 01-.277.71A6.975 6.975 0 0110 18a6.974 6.974 0 01-4.696-1.81z" />
                      </svg>
                      Team Libraries
                    </dt>
                    <dd className="inline">
                      {" "}
                      Build a library of reusable actions and templates that your entire team can access.
                    </dd>
                  </div>
                </dl>
              </div>
            </div>
            <div className="order-2">
              <Image
                src="/images/share.png"
                alt="Screenshot showing team sharing interface"
                className="w-full max-w-[35.2rem] mx-auto rounded-xl shadow-xl ring-1 ring-gray-400/10 sm:w-[52.8rem]"
                width={880}
                height={660}
              />
            </div>
          </div>
        </div>
      </div>
      <div className="overflow-hidden bg-white py-12 sm:py-32">
        <div className="mx-auto max-w-7xl px-6 lg:px-8">
          <div className="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-8 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
            <div className="order-2 lg:order-1">
              <Image
                src="/images/monitor.png"
                alt="Screenshot showing monitoring and analytics dashboard"
                className="w-full max-w-[35.2rem] mx-auto rounded-xl shadow-xl ring-1 ring-gray-400/10 sm:w-[52.8rem]"
                width={880}
                height={660}
              />
            </div>
            <div className="lg:pl-8 lg:pt-4 order-1 lg:order-2">
              <div className="lg:max-w-lg">
                <h2 className="text-base font-semibold leading-7 text-purple-600">
                  Monitoring & Analytics
                </h2>
                <p className="mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl">
                  Track automation performance
                </p>
                <p className="mt-6 text-lg leading-8 text-gray-600">
                  Monitor your automation performance in real-time. Get detailed insights into workflow execution, errors, and optimization opportunities.
                </p>
                <dl className="mt-10 max-w-xl space-y-8 text-base leading-7 text-gray-600 lg:max-w-none">
                  <div className="relative pl-9">
                    <dt className="inline font-semibold text-gray-900">
                      <svg
                        className="absolute left-1 top-1 h-5 w-5 text-purple-600"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                      >
                        <path d="M15.5 2A1.5 1.5 0 0014 3.5v13a1.5 1.5 0 001.5 1.5h1a1.5 1.5 0 001.5-1.5v-13A1.5 1.5 0 0016.5 2h-1zM9.5 6A1.5 1.5 0 008 7.5v9A1.5 1.5 0 009.5 18h1a1.5 1.5 0 001.5-1.5v-9A1.5 1.5 0 0010.5 6h-1zM3.5 10A1.5 1.5 0 002 11.5v5A1.5 1.5 0 003.5 18h1A1.5 1.5 0 006 16.5v-5A1.5 1.5 0 004.5 10h-1z" />
                      </svg>
                      Real-time Analytics
                    </dt>
                    <dd className="inline">
                      {" "}
                      Get instant visibility into your automation performance and execution metrics.
                    </dd>
                  </div>
                </dl>
              </div>
            </div>
          </div>
        </div>
      </div>
      <PricingGroup />
    </>
  );
}
