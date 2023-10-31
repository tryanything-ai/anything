import {
  fetchProfile,
  fetchTemplateBySlug,
  flowJsonFromBigFlow,
  Profile,
} from "utils";
import { ImageResponse } from "next/server";
import { FlowTemplateOgImage } from "@/components/og/template2";
import { FlowTemplate } from "@/types/flow";
// import fs from "fs";
// import path from "path";
// import { fileURLToPath } from "url";

const this_url = process.env.NEXT_PUBLIC_VERCEL_URL;
// import { dm_sans } from "@/lib/fonts";
// import { DM_Sans, Inter } from "next/font/google";

// export const inter = Inter({
//   subsets: ["latin"],
//   display: "swap",
//   variable: "--font-inter",
// });

// export const dm_sans = DM_Sans({
//   weight: ["400", "500", "700"],
//   subsets: ["latin"],
//   display: "swap",
//   variable: "--font-dm-sans",
// });

// Route segment config
export const runtime = "edge";

// Image metadata
export const alt = "Anything Template";
export const size = {
  width: 1200,
  height: 628,
};

export const contentType = "image/png";

// console.log("import meta", JSON.stringify(import.meta));

// const boldFont = fetch("/fonts/DMSans-SemiBold.ttf").then((res) =>
//   res.arrayBuffer()
// );

// const boldFont = fetch(
//   new URL("../../../../../public/fonts/DMSans-SemiBold.ttf", import.meta.url)
// ).then((res) => res.arrayBuffer());

// const boldFont = fs.promises.readFile(
//   path.join(
//     fileURLToPath(import.meta.url),
//     "../../../../../public/fonts/DMSans-SemiBold.ttf"
//   )
// );

// Image generation
export default async function Image({
  params,
}: {
  params: { slug: string };
}): Promise<ImageResponse> {
  console.log(
    "params in TemplatePageOgImage Generation",
    JSON.stringify(params)
  );
  const templateResponse = await fetchTemplateBySlug(params.slug);

  if (!templateResponse) {
    console.log(
      "templateResponse in TemplatePage",
      JSON.stringify(templateResponse, null, 3)
    );
    throw new Error("Template not found");
  }

  const template = templateResponse[0];
  console.log("template in TemplatePage", JSON.stringify(template, null, 3));

  const profile: Profile | undefined = template?.profiles?.username
    ? await fetchProfile(template.profiles.username)
    : undefined;

  const flow = (await flowJsonFromBigFlow(template)) as FlowTemplate;

  console.log(
    "params in TemplatePageOgImage Generation",
    JSON.stringify(params)
  );

  //  // Font
  //  const interSemiBold = fetch(
  //   new URL('./Inter-SemiBold.ttf', import.meta.url)
  //  ).then((res) => res.arrayBuffer())

  // const dmSansFontResponse = await fetch(
  //   process.env.NEXT_PUBLIC_VERCEL_URL + "/fonts/DM_Sans.ttf"
  // );
  // const dmSansFontBuffer = await dmSansFontResponse.arrayBuffer();
  // const [regularFontData, boldFontData] = await Promise.all([
  //   regularFont,
  //   boldFont
  // ]);

  const boldFontData = await fetch(
    this_url + "/fonts/DMSans-SemiBold.ttf"
  ).then((res) => res.arrayBuffer());

  // const boldFontData = await boldFont;
  // const boldFontData = await fetch(
  //   new URL("../../../../../public/fonts/DMSans-SemiBold.ttf", process.env.NEXT_PUBLIC_VERCEL_URL)
  // ).then((res) => res.arrayBuffer());

  return new ImageResponse(
    (
      <div
        style={{
          height: "100%",
          width: "100%",
          display: "flex",
          // flexDirection: 'column',
          // alignItems: 'center',
          // justifyContent: 'center',
          // backgroundColor: 'white',
          // fontFamily: "derp",
          fontFamily: "Dm_Sans",
          // fontWeight: 700,
        }}
      >
        <FlowTemplateOgImage
          actions={flow.actions}
          profileImage={profile?.avatar_url || ""}
          profileName={profile?.full_name || ""}
          title={template.flow_template_name}
          trigger={flow.trigger}
          username={profile?.username || ""}
        />
      </div>
    ),
    {
      ...size,
      fonts: [
        {
          name: "Dm_Sans",
          data: boldFontData,
          // style: "normal",
          weight: 700,
        },
      ],
      // fonts: [
      // {
      //   name: 'DM Sans',
      //   data: await dm_sans,
      //   style: 'normal',
      //   weight: 500,
      //   subsets: ["latin"],
      //   display: "swap",
      //   variable: "--font-dm-sans",
      // },
      // {
      //   name: 'Inter',
      //   data: await inter,
      //   style: 'normal',
      //   weight: 400,
      //   subsets: ["latin"],
      //   display: "swap",
      //   variable: "--font-inter",
      //         weight: 500,
      // subsets: ["latin"],
      // display: "swap",
      // variable: "--font-dm-sans",

      // },
      // ]
    }
  );
}

/* <FlowTemplateOgImage
          actions={flow.actions}
          profileImage={profile?.avatar_url || ""}
          profileName={profile?.full_name || ""}
          title={template.flow_template_name}
          trigger={flow.trigger}
          username={profile?.username || ""}
        /> */
// For convenience, we can re-use the exported opengraph-image
// size config to also set the ImageResponse's width and height.
// ...size,
// fonts: [
//   {
//     name: 'Inter',
//     data: await interSemiBold,
//     style: 'normal',
//     weight: 400,
//   },
// ],
