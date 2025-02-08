import { TemplateView } from "@repo/ui/components/templateView";
import { getAProfileLink } from "@repo/ui/helpers/helpers";
import type { Metadata, ResolvingMetadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import { Avatar } from "@/components/avatar";
import api, { DBFlowTemplate } from "@repo/anything-api";

type Props = {
  params: { slug: string };
};

export async function generateMetadata(
  { params }: Props,
  parent: ResolvingMetadata,
): Promise<Metadata> {
  let Metadata: Metadata = {};

  try {
    // fetch data
    const templateResponse =
      await api.marketplace.getWorkflowTemplateBySlugForMarketplace(params.slug);

    // Check if templateResponse is undefined, null, or doesn't have required fields
    if (!templateResponse || !templateResponse.flow_template_name) {
      console.error("Template not found or invalid");
      return Metadata;
    }

    const template: DBFlowTemplate = templateResponse;

    Metadata = {
      title: template.flow_template_name,
      description: template?.flow_template_description || "Workflow Template",
      openGraph: {
        description: template?.flow_template_description ?? "Workflow Template",
      },
      twitter: {
        description: template?.flow_template_description ?? "Workflow Template",
      },
      authors: [{}],
    };
  } catch (error) {
    console.error("Error generating metadata:", error);
  }

  return Metadata;
}

export const generateStaticParams = async (): Promise<Array<Props["params"]>> => {
  try {
    const templates = await api.marketplace.getWorkflowTemplatesForMarketplace();
    if (!templates || !Array.isArray(templates)) return [];
    
    return templates.map(template => ({
      slug: template.slug || String(template.flow_template_id)
    }));
  } catch (error) {
    console.error("Error generating static params:", error);
    return [];
  }
};

const Action = ({ template, profile }: { template: DBFlowTemplate; profile: any }): JSX.Element => {
  return (
    <div className="flex justify-center">
      <a
        className="inline-block px-6 py-3 text-lg font-semibold text-white bg-purple-600 rounded-full shadow-lg hover:bg-pink-600  transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105"
        data-ph-capture-attribute-flow-template-id={template?.flow_template_id}
        data-ph-capture-attribute-flow-template-name={template?.flow_template_name}
        data-ph-capture-attribute-flow-template-profile-id={profile?.id}
        data-ph-capture-attribute-flow-template-profile-username={profile?.username}
        data-ph-capture-attribute-flow-template-slug={template?.slug}
        href={`https://app.tryanything.xyz/templates/${template?.flow_template_id}`}
      >
        Use This Template
      </a>
    </div>
  );
};

export default async function Template({
  params,
}: Props): Promise<JSX.Element> {
  try {
    const templateResponse =
      await api.marketplace.getWorkflowTemplateBySlugForMarketplace(params.slug);

    if (!templateResponse || !templateResponse.flow_template_id) {
      console.error("Template not found or invalid");
      notFound();
    }

    const template: DBFlowTemplate = templateResponse;
    const profile: any | undefined = template?.profiles;

    return (
      <div className="mx-4 my-6 flex max-w-4xl flex-col md:mx-auto md:my-16">
        <TemplateView
          ActionComponent={Action}
          Avatar={Avatar}
          Link={Link as any}
          profile={profile}
          template={template}
        />
      </div>
    );
  } catch (error) {
    console.error("Error rendering template:", error);
    notFound();
  }
}
