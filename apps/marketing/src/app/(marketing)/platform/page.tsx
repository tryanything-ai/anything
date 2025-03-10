import { headers } from "next/headers";

// import { SignUpButton } from "@/components/marketing/LandingSignUp";
import Image from "next/image";
import { Button } from "@repo/ui/components/ui/button";
import AnythingWeb from "../../../../public/anything_web.png";
import PlatformFeatures from "@/components/PlatformFeatures";
import PlatformFaq from "@/components/PlatformFaq";

export default function IndexPage() {
  // const headerList = headers();`
  // const referringDomain = headerList.get("referer");

  return (
    <div>
      {/* Bg Pattern */}
      {/* <BgPattern /> */}
      {/* Hero Copy */}
      <div className="mt-16 flex flex-col items-center gap-4">
        {/*  */}
        {/* <h1 className="font-display text-[80px] font-semibold leading-[88px] tracking-[-2%] h2 w-full px-4 text-center md:w-[802px] md:px-0"> */}
        <h1 className="text-white md:display h1 w-full px-4 text-center md:w-[800px] md:px-0 max-w-5xl">
          Build an <span className="text-crimson-9">Automation Platform</span>{" "}
          for your industry
        </h1>
        <p className="mt-2 text-white body-xl px-4 text-center md:py-5 md:w-[705px] md:px-0 max-w-xl">
          ... and launch this week.
        </p>

        {/* <p className="mt-2 body-xl px-4 text-center md:py-5 text-slate-11 md:w-[705px] md:px-0 max-w-xl">
          Zapier, Make and N8N are big and complicated and <span className="text-white">you don't own them.</span>
          <div className="mt-4" />
          Build an automation platform catering to your niche and launch this week.
        </p> */}
        {/* <p className="body-xl px-4 text-center md:py-5 text-slate-11 md:w-[705px] md:px-0">
          Build an automation catering to your niche. Minimal engineering effort.
        </p> */}
        {/* <p className="body-xl px-4 text-center md:py-5 text-slate-11 md:w-[705px] md:px-0">
          Think of it like Shopify but for building an Automation Business.
        </p> */}
      </div>
      {/* Hero CTA */}
      <div className="mb-20 mt-20 flex flex-col items-center gap-4">
        {/* <p className="body">
          Get your <span className="font-semibold">free account today</span>
        </p> */}
        <div className="flex flex-col w-full px-4 items-center justify-center md:flex-row gap-4">
          {/* <SignUpButton className="block" />
           */}
          {/* <Button
            href={`https://cal.com/carllippert/anything-ai-platform-intro-call`}
            rel="noopener noreferrer"
            target="_blank"
            variant="daisy_primary"
          >
            Get In Touch
          </Button> */}
          {/* <Button
            href={`https://airtable.com/shrfQYBtcoUqYNylu?prefill_fldVLaD0gtTpY1jxP=wysiwyg&hide_fldVLaD0gtTpY1jxP=true&prefill_referring_domain=${referringDomain}&hide_referring_domain=true`}
            rel="noopener noreferrer"
            target="_blank"
            variant="daisy_outline"
          >
            Talk To Sales
          </Button> */}
        </div>
        {/* <p className="caption text-slate-11">No credit card required</p> */}
      </div>
      {/* <div className="flex flex-col w-full max-w-5xl mx-auto">
        <Image
          src={AnythingWeb}

          alt="Anything Web Screenshot"
        // width="500"
        // height="1000"
        />
      </div> */}
      <div
        className="relative w-full max-w-5xl mx-auto"
        style={{ paddingBottom: "62.5%", height: 0 }}
      >
        <iframe
          src="https://www.loom.com/embed/ee679ad928914ccc984781bb231e2ca2?sid=b5294062-65f6-43b8-b1fe-24120dfcfc69"
          frameBorder="0"
          allowFullScreen
          title="Anything AI Demo"
          className="absolute top-0 left-0 w-full h-full sm:px-0 px-6"
        ></iframe>
      </div>
      <PlatformFeatures />
      {/* TODO: Add Features List */}
      {/* Templates https://tailwindui.com/components/marketing/sections/feature-sections */}
      {/* <div className="relative w-full max-w-5xl mx-auto"> */}
      {/* <h2 className="text-2xl text-center mb-12 font-semibold text-slate-12">Demo Video</h2> */}
      {/* <div className="relative w-full max-w-5xl mx-auto" style={{ paddingBottom: "62.5%", height: 0 }}>

        <iframe
          src="https://www.loom.com/embed/ee679ad928914ccc984781bb231e2ca2?sid=b5294062-65f6-43b8-b1fe-24120dfcfc69"
          frameBorder="0"
          allowFullScreen
          title="Anything AI Demo"
          className="absolute top-0 left-0 w-full h-full sm:px-0 px-6"
        ></iframe>
      </div> */}
      <PlatformFaq />
    </div>
  );
}
