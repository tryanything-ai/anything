// import { flowJsonFromBigFlow } from "@repo/ui/helpers/helpers";
import { ImageResponse } from "next/og";
import { FlowTemplateOgImage } from "@/components/og/template_css";
// import { FlowTemplate } from "@/types/flow";
import api, { DBFlowTemplate } from "@repo/anything-api";
const this_url = "http://" + process.env.VERCEL_PROJECT_PRODUCTION_URL;

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
export default async function Image({ params }: { params: { slug: string } }) {
  console.log(
    "params in TemplatePageOgImage Generation",
    JSON.stringify(params),
  );
  const templateResponse =
    await api.marketplace.getWorkflowTemplateBySlugForMarketplace(params.slug);

  if (!templateResponse) {
    console.log(
      "templateResponse in TemplatePage",
      JSON.stringify(templateResponse, null, 3),
    );
    throw new Error("Template not found");
  }

  const template: DBFlowTemplate = templateResponse[0];
  console.log("template in TemplatePage", JSON.stringify(template, null, 3));

  const profile: any | undefined = template?.profiles?.username
    ? await api.profiles.getMarketplaceProfileByUsername(
        template.profiles.username,
      )
    : undefined;

  // const flow = (await flowJsonFromBigFlow(template)) as FlowTemplate;

  const getFlowDetails = (template: DBFlowTemplate) => {
    const latestVersion = template.flow_template_versions[0];
    if (!latestVersion || !latestVersion.flow_definition) {
      return { trigger: null, actions: [] };
    }

    const { actions } = latestVersion.flow_definition;
    const trigger = actions.find((action) => action.type === "trigger");
    const nonTriggerActions = actions.filter(
      (action) => action.type !== "trigger",
    );

    return { trigger, actions: nonTriggerActions };
  };

  const { trigger, actions } = getFlowDetails(template);

  console.log(
    "params in TemplatePageOgImage Generation",
    JSON.stringify(params),
  );

  const boldFontData = await fetch(
    this_url + "/fonts/DMSans-SemiBold.ttf",
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
        {/* TODO: bring back */}
        <FlowTemplateOgImage
          actions={actions}
          profileImage={profile?.avatar_url || ""}
          profileName={profile?.full_name || ""}
          title={template.flow_template_name}
          trigger={trigger}
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
    },
  );
}
