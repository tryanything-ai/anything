import { headers } from "next/headers";
import { Button } from "@repo/ui/components/ui/button";
import Llama from "../../../public/llamascreenshot.png";
import Features from "@/components/LocalFeatures";
import Faq from "@/components/LocalFaq";
import Image from "next/image";
import HeroVideoDialog from "@repo/ui/components/magicui/hero-video-dialog";
import ShimmerButton from "@repo/ui/components/magicui/shimmer-button";
import Link from "next/link";
import PricingCalculator from "@/components/PricingCalculator";

const ConsultingLink = "https://calendar.app.google/9gWy5xtDv3YkujAi7";

export default function IndexPage() {
  const headerList = headers();

  return (
    <>
      <div className="mt-16 flex flex-col items-center gap-4">
        <h1 className="text-5xl md:text-7xl font-bold text-center max-w-4xl px-4 md:px-0">
          Never Miss a Lead with{" "}
          <span className="text-purple-600">Instant AI Response</span>
        </h1>
        <p className="text-xl md:text-2xl text-center text-gray-600 max-w-2xl px-4 md:px-0 mt-4">
          Qualify inbound leads within seconds and manage your calendar like a
          pro. Our AI agents respond instantly to inquiries and handle
          scheduling 24/7.
        </p>
        <div className="flex gap-4 mt-8">
          {/* <Link
            href="/get-started"
            className="bg-purple-600 text-white px-6 py-3 rounded-lg font-bold hover:bg-purple-700 transition-colors"
          >
            Start Free Trial
          </Link> */}
          <Link
            href={ConsultingLink}
            className="border border-purple-600 text-purple-600 px-6 py-3 rounded-lg font-bold hover:bg-purple-50 transition-colors"
          >
            Schedule Demo
          </Link>
        </div>
      </div>

      <div className="mt-24">
        <h2 className="text-3xl font-bold text-center mb-12">
          Your 24/7 AI Voice Team
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-7xl mx-auto px-6">
          <div className="flex flex-col items-center text-center group">
            <div className="w-32 h-32 rounded-full mb-4 overflow-hidden transition-transform duration-300 group-hover:scale-105 bg-purple-100">
              <div className="w-24 h-24 rounded-full overflow-hidden mx-auto mt-4">
                <Image
                  src="/images/agents/alexa.jpg"
                  alt="Sarah - AI Lead Qualifier"
                  width={128}
                  height={128}
                  className="w-full h-full object-cover"
                />
              </div>
            </div>
            <h3 className="text-xl font-semibold mb-2">Sarah</h3>
            <p className="text-gray-600">Instant Lead Qualifier</p>
            <div className="mt-4 space-y-2">
              <p className="text-sm text-gray-600">
                • Responds in seconds to new leads
              </p>
              <p className="text-sm text-gray-600">
                • Qualifies based on your criteria
              </p>
              <p className="text-sm text-gray-600">
                • Routes hot leads to sales instantly
              </p>
            </div>
            <div className="mt-4 w-full bg-gray-100 p-4 rounded-lg hover:bg-gray-200 transition-colors cursor-pointer">
              <div className="flex items-center justify-center gap-2">
                <svg
                  className="w-5 h-5 text-purple-600"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path d="M18 3a1 1 0 00-1.196-.98l-10 2A1 1 0 006 5v9.114A4.369 4.369 0 005 14c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V7.82l8-1.6v5.894A4.37 4.37 0 0015 12c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V3z" />
                </svg>
                <span className="text-sm font-medium">Listen to sample</span>
              </div>
            </div>
          </div>

          <div className="flex flex-col items-center text-center group">
            <div className="w-32 h-32 rounded-full mb-4 overflow-hidden transition-transform duration-300 group-hover:scale-105 bg-purple-100">
              <div className="w-24 h-24 rounded-full overflow-hidden mx-auto mt-4">
                <Image
                  src="/images/agents/june.jpg"
                  alt="Emma - AI Secretary"
                  width={128}
                  height={128}
                  className="w-full h-full object-cover"
                />
              </div>
            </div>
            <h3 className="text-xl font-semibold mb-2">Emma</h3>
            <p className="text-gray-600">AI Secretary</p>
            <div className="mt-4 space-y-2">
              <p className="text-sm text-gray-600">
                • Answers common FAQs 24/7
              </p>
              <p className="text-sm text-gray-600">
                • Schedules appointments seamlessly
              </p>
              <p className="text-sm text-gray-600">
                • Integrates with your calendar
              </p>
            </div>
            <div className="mt-4 w-full bg-gray-100 p-4 rounded-lg hover:bg-gray-200 transition-colors cursor-pointer">
              <div className="flex items-center justify-center gap-2">
                <svg
                  className="w-5 h-5 text-purple-600"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path d="M18 3a1 1 0 00-1.196-.98l-10 2A1 1 0 006 5v9.114A4.369 4.369 0 005 14c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V7.82l8-1.6v5.894A4.37 4.37 0 0015 12c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V3z" />
                </svg>
                <span className="text-sm font-medium">Listen to sample</span>
              </div>
            </div>
          </div>

          <div className="flex flex-col items-center text-center group">
            <div className="w-32 h-32 rounded-full mb-4 overflow-hidden transition-transform duration-300 group-hover:scale-105 bg-purple-100">
              <div className="w-24 h-24 rounded-full overflow-hidden mx-auto mt-4">
                <Image
                  src="/images/agents/derick.jpg"
                  alt="Michael - AI Sales Outreach"
                  width={128}
                  height={128}
                  className="w-full h-full object-cover"
                />
              </div>
            </div>
            <h3 className="text-xl font-semibold mb-2">Michael</h3>
            <p className="text-gray-600">AI Sales Outreach</p>
            <div className="mt-4 space-y-2">
              <p className="text-sm text-gray-600">
                • Makes outbound sales calls
              </p>
              <p className="text-sm text-gray-600">
                • Follows your sales script
              </p>
              <p className="text-sm text-gray-600">
                • Books meetings with qualified leads
              </p>
            </div>
            <div className="mt-4 w-full bg-gray-100 p-4 rounded-lg hover:bg-gray-200 transition-colors cursor-pointer">
              <div className="flex items-center justify-center gap-2">
                <svg
                  className="w-5 h-5 text-purple-600"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path d="M18 3a1 1 0 00-1.196-.98l-10 2A1 1 0 006 5v9.114A4.369 4.369 0 005 14c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V7.82l8-1.6v5.894A4.37 4.37 0 0015 12c-1.657 0-3 .895-3 2s1.343 2 3 2 3-.895 3-2V3z" />
                </svg>
                <span className="text-sm font-medium">Listen to sample</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="mt-24">
        <div className="max-w-7xl mx-auto px-6">
          <h2 className="text-3xl font-bold text-center mb-12">
            Lightning-Fast Lead Response & Seamless Scheduling
          </h2>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-12">
            <div className="space-y-6">
              <h3 className="text-2xl font-bold">
                Convert More Leads with Instant Response
              </h3>
              <ul className="space-y-4">
                <li className="flex items-center gap-3">
                  <svg
                    className="w-5 h-5 text-purple-600"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                  >
                    <path
                      fillRule="evenodd"
                      d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                      clipRule="evenodd"
                    />
                  </svg>
                  Responds in under 5 seconds
                </li>
                <li className="flex items-center gap-3">
                  <svg
                    className="w-5 h-5 text-purple-600"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                  >
                    <path
                      fillRule="evenodd"
                      d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                      clipRule="evenodd"
                    />
                  </svg>
                  Qualifies leads with natural conversation
                </li>
                <li className="flex items-center gap-3">
                  <svg
                    className="w-5 h-5 text-purple-600"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                  >
                    <path
                      fillRule="evenodd"
                      d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                      clipRule="evenodd"
                    />
                  </svg>
                  Instantly routes qualified leads to sales
                </li>
                <li className="flex items-center gap-3">
                  <svg
                    className="w-5 h-5 text-purple-600"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                  >
                    <path
                      fillRule="evenodd"
                      d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                      clipRule="evenodd"
                    />
                  </svg>
                  24/7 availability for global coverage
                </li>
              </ul>
            </div>
            <div>
              <Image
                src="/images/test_and_deploy.png"
                alt="AI Lead Response Dashboard"
                className="rounded-xl shadow-xl"
                width={880}
                height={660}
              />
            </div>
          </div>
        </div>
      </div>

      <div className="mt-24 bg-gray-50 py-24">
        <div className="max-w-7xl mx-auto px-6">
          <h2 className="text-3xl font-bold text-center mb-4">
            Simple, Transparent Pricing
          </h2>
          <p className="text-xl text-center text-gray-600 mb-12">
            Start with our base plan at $10/month and only pay for the minutes
            you use
          </p>

          <PricingCalculator />
        </div>
      </div>

      <div className="mt-24">
        <div className="max-w-7xl mx-auto px-6">
          <h2 className="text-3xl font-bold text-center mb-12">
            Why Speed-to-Lead Matters
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            <div className="bg-white p-6 rounded-xl shadow-sm">
              <div className="w-12 h-12 bg-purple-100 rounded-lg mb-4 flex items-center justify-center">
                <svg
                  className="w-6 h-6 text-purple-600"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path d="M2 10a8 8 0 018-8v8h8a8 8 0 11-16 0z" />
                  <path d="M12 2.252A8.014 8.014 0 0117.748 8H12V2.252z" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold mb-2">
                78% Higher Conversion
              </h3>
              <p className="text-gray-600">
                Leads contacted within 5 minutes are 78% more likely to convert
                than those contacted after 30 minutes
              </p>
            </div>

            <div className="bg-white p-6 rounded-xl shadow-sm">
              <div className="w-12 h-12 bg-purple-100 rounded-lg mb-4 flex items-center justify-center">
                <svg
                  className="w-6 h-6 text-purple-600"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fillRule="evenodd"
                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z"
                    clipRule="evenodd"
                  />
                </svg>
              </div>
              <h3 className="text-xl font-semibold mb-2">100% Response Rate</h3>
              <p className="text-gray-600">
                Never miss another lead with instant response to every inquiry,
                any time of day
              </p>
            </div>

            <div className="bg-white p-6 rounded-xl shadow-sm">
              <div className="w-12 h-12 bg-purple-100 rounded-lg mb-4 flex items-center justify-center">
                <svg
                  className="w-6 h-6 text-purple-600"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fillRule="evenodd"
                    d="M3.172 5.172a4 4 0 015.656 0L10 6.343l1.172-1.171a4 4 0 115.656 5.656L10 17.657l-6.828-6.829a4 4 0 010-5.656z"
                    clipRule="evenodd"
                  />
                </svg>
              </div>
              <h3 className="text-xl font-semibold mb-2">
                Improved Experience
              </h3>
              <p className="text-gray-600">
                Provide instant, professional responses that make prospects feel
                valued
              </p>
            </div>
          </div>
        </div>
      </div>

      <div className="mt-24">
        <div className="max-w-7xl mx-auto px-6">
          <h2 className="text-3xl font-bold text-center mb-12">
            Set Up in Minutes
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
            <div className="text-center">
              <div className="w-16 h-16 bg-purple-100 rounded-full mx-auto mb-4 flex items-center justify-center">
                <span className="text-2xl font-bold text-purple-600">1</span>
              </div>
              <h3 className="text-xl font-semibold mb-2">Connect</h3>
              <p className="text-gray-600">Link your lead forms and calendar</p>
            </div>

            <div className="text-center">
              <div className="w-16 h-16 bg-purple-100 rounded-full mx-auto mb-4 flex items-center justify-center">
                <span className="text-2xl font-bold text-purple-600">2</span>
              </div>
              <h3 className="text-xl font-semibold mb-2">Configure</h3>
              <p className="text-gray-600">Set your qualification criteria</p>
            </div>

            <div className="text-center">
              <div className="w-16 h-16 bg-purple-100 rounded-full mx-auto mb-4 flex items-center justify-center">
                <span className="text-2xl font-bold text-purple-600">3</span>
              </div>
              <h3 className="text-xl font-semibold mb-2">Customize</h3>
              <p className="text-gray-600">Personalize your AI agent</p>
            </div>

            <div className="text-center">
              <div className="w-16 h-16 bg-purple-100 rounded-full mx-auto mb-4 flex items-center justify-center">
                <span className="text-2xl font-bold text-purple-600">4</span>
              </div>
              <h3 className="text-xl font-semibold mb-2">Launch</h3>
              <p className="text-gray-600">Start converting leads instantly</p>
            </div>
          </div>
        </div>
      </div>

      <div className="mt-24 text-center">
        <h2 className="text-3xl font-bold mb-8">
          Start Converting More Leads Today
        </h2>
        <div className="flex justify-center gap-4">
          {/* <Link
            href="/get-started"
            className="bg-purple-600 text-white px-6 py-3 rounded-lg font-bold hover:bg-purple-700 transition-colors"
          >
            Start Free Trial
          </Link> */}
          <Link
            href={ConsultingLink}
            className="border border-purple-600 text-purple-600 px-6 py-3 rounded-lg font-bold hover:bg-purple-50 transition-colors"
          >
            Schedule Demo
          </Link>
        </div>
      </div>
    </>
  );
}
