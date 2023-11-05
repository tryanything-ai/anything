import { headers } from "next/headers";

// import { BgPattern } from "@/components/ui/Bgpattern";
// import { SignUpButton } from "@/components/marketing/LandingSignUp";
import Image from "next/image";
import { Button } from "@/components/ui/Button";
import Llama from "../../../public/llamascreenshot.png";

export default function IndexPage() {
  const headerList = headers();
  const referringDomain = headerList.get("referer");

  return (
    <>
      {/* Bg Pattern */}
      {/* <BgPattern /> */}
      {/* Hero Copy */}
      <div className="mt-16 flex flex-col items-center gap-4">
        {/*  */}
        {/* <h1 className="font-display text-[80px] font-semibold leading-[88px] tracking-[-2%] h2 w-full px-4 text-center md:w-[802px] md:px-0"> */}
        <h1 className="md:display h3 w-full px-4 text-center md:w-[800px] md:px-0">
          Build <span className="text-crimson-9">AI Automations</span> for your
          startup
        </h1>
        <p className="body-xl px-4 text-center md:py-5 text-slate-11 md:w-[705px] md:px-0">
          Run Locally. Customize. Self host.
        </p>
      </div>
      {/* Hero CTA */}
      <div className="mb-20 mt-20 flex flex-col items-center gap-4">
        {/* <p className="body">
          Get your <span className="font-semibold">free account today</span>
        </p> */}
        <div className="flex flex-col w-full px-4 items-center justify-center md:flex-row gap-4">
          {/* <SignUpButton className="block" />
           */}
          <Button
            href={`https://airtable.com/shrfQYBtcoUqYNylu?prefill_fldVLaD0gtTpY1jxP=wysiwyg&hide_fldVLaD0gtTpY1jxP=true&prefill_referring_domain=${referringDomain}&hide_referring_domain=true`}
            rel="noopener noreferrer"
            target="_blank"
            variant="daisy_primary"
          >
            Get Early Access
          </Button>
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
      <div className="flex flex-col w-full max-w-5xl mx-auto">
        <Image
          src={Llama}
          // src={`https://${process.env.NEXT_PUBLIC_VERCEL_URL}/llamascreenshot.png`}
          alt="Llama Screenshot"
          // width="500"
          // height="1000"
        />
      </div>
    </>
  );
}
