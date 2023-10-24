import { flowJsonFromBigFlow } from "@anything/utils";
import { ImageResponse } from "next/server";

import { FlowTemplateOgImage } from "@/components/og/template";
import { fetchProfile, fetchTemplateBySlug , Profile } from "@/lib/fetchSupabase";
import { FlowTemplate } from "@/types/flow";

// Route segment config
export const runtime = "edge";

// Image metadata
export const alt = "Anything Template";
export const size = {
  width: 1200,
  height: 630,
};

export const contentType = "image/png";

// Image generation
export default async function Image({ params }: { params: { slug: string } }) {
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

  let template = templateResponse[0];
  console.log("template in TemplatePage", JSON.stringify(template, null, 3));

  let profile: Profile | undefined = template?.profiles?.username
    ? await fetchProfile(template.profiles.username)
    : undefined;

  let flow = flowJsonFromBigFlow(template) as FlowTemplate;
 
  return new ImageResponse(
    (
      <FlowTemplateOgImage
        title={template.flow_template_name}
        username={profile?.username || ""}
        profileName={profile?.full_name || ""}
        profileImage={profile?.avatar_url || ""}
        trigger={flow.trigger}
        actions={flow.actions}
      />
    ),
    {
      
      // For convenience, we can re-use the exported opengraph-image
      // size config to also set the ImageResponse's width and height.
      ...size,

      // fonts: [
      //   {
      //     name: 'Inter',
      //     data: await interSemiBold,
      //     style: 'normal',
      //     weight: 400,
      //   },
      // ],
    }
  );
}
