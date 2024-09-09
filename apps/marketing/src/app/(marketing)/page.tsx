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
  const referringDomain = headerList.get("referer");

  return (
    <>
      <div className="mt-16 flex flex-col items-center gap-4">
        <h1 className="text-5xl md:text-7xl font-bold text-center max-w-4xl px-4 md:px-0">
          Build <span className="text-purple-600">AI Automations</span> to
          Supercharge Your Business
        </h1>
        <p className="text-xl md:text-2xl text-center text-gray-600 max-w-2xl px-4 md:px-0 mt-4">
          Save time. Reduce errors. Automate drudgery.
        </p>
      </div>
      <div className="max-w-7xl mx-auto px-5 mt-10">
        <HeroVideoDialog
          className="dark:hidden block"
          animationStyle="from-center"
          videoSrc="https://www.youtube.com/embed/qh3NGpYRG3I?si=4rb-zSdDkVK9qxxb"
          thumbnailSrc="https://startup-template-sage.vercel.app/hero-light.png"
          thumbnailAlt="Hero Video"
        />
        {/* <HeroVideoDialog
          className="hidden dark:block"
          animationStyle="from-center"
          videoSrc="https://www.youtube.com/embed/qh3NGpYRG3I?si=4rb-zSdDkVK9qxxb"
          thumbnailSrc="https://startup-template-sage.vercel.app/hero-dark.png"
          thumbnailAlt="Hero Video"
        /> */}
      </div>
      {/* <div className="mt-20 mb-20 flex flex-col items-center gap-4">
        <div className="flex flex-col w-full px-4 items-center justify-center md:flex-row gap-4">
          {/* Add your buttons here if needed */}
      {/* </div> */}
      {/* </div> */}
      {/* <div className="flex flex-col w-full max-w-5xl mx-auto">
        <Image src={Llama} alt="Llama Screenshot" />
      </div> */}
      <PricingGroup />
    </>
  );
}
