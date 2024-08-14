import {
    fetchProfile,
    fetchTemplateBySlug,
  } from "@/lib/supabase/fetchSupabase";
  import { flowJsonFromBigFlow } from "@repo/ui/helpers/helpers";
  import { ImageResponse } from "next/og";
  import { FlowTemplateOgImage } from "@/components/og/template_css";
  import { FlowTemplate } from "@/types/flow";
  
  const this_url = "http://" + process.env.NEXT_PUBLIC_VERCEL_URL;
  
  // Route segment config
  export const runtime = "edge";
  
  // Image metadata
  export const alt = "Anything Template";
  export const size = {
    width: 1200,
    height: 628,
  };
  
  export const contentType = "image/png";
  
  // Image generation
  export default async function Image({
    params,
  }: {
    params: { slug: string };
  }) {
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
  
    const profile: any | undefined = template?.profiles?.username
      ? await fetchProfile(template.profiles.username)
      : undefined;
  
    const flow = (await flowJsonFromBigFlow(template)) as FlowTemplate;
  
    console.log(
      "params in TemplatePageOgImage Generation",
      JSON.stringify(params)
    );
  
    const boldFontData = await fetch(
      this_url + "/fonts/DMSans-SemiBold.ttf"
    ).then((res) => res.arrayBuffer());
  
    return new ImageResponse(
      (
        <div
          style={{
            height: "100%",
            width: "100%",
            display: "flex",
            fontFamily: "Dm_Sans",
            backgroundColor: "black",
            color: "#FFFFFF",
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
            weight: 700,
          },
        ],
      }
    );
  }
  