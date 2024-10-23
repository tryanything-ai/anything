import { TemplateView } from "@repo/ui/components/templateView";
import { getAProfileLink } from "@repo/ui/helpers/helpers";
import type { Metadata, ResolvingMetadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import { Avatar } from "@/components/avatar";
import api, { DBFlowTemplate } from "@repo/anything-api";

type Props = {
  params: { slug: string };
  // searchParams: { [key: string]: string | string[] | undefined };
};

export async function generateMetadata(
  { params }: Props,
  parent: ResolvingMetadata,
): Promise<Metadata> {
  let Metadata: Metadata = {};

  // fetch data
  const templateResponse =
    await api.marketplace.getWorkflowTemplateBySlugForMarketplace(params.slug);

  // Print the response for debugging purposes
  console.log("Template Response for gernateMetadata:", templateResponse);

  // Check if templateResponse is undefined or null
  if (!templateResponse) {
    console.error("Template not found");
    notFound();
  }

  if (templateResponse) {
    const template: DBFlowTemplate = templateResponse;

    // const profile: any | undefined = template?.profiles?.username
    //   ? await fetchProfile(template.profiles.username)
    //   : undefined;

    Metadata = {
      title: template.flow_template_name,
      description: template?.flow_template_description,
      openGraph: {
        description: template?.flow_template_description ?? undefined,
      },
      twitter: {
        description: template?.flow_template_description ?? undefined,
      },
      authors: [
        {
          // name: profile?.full_name || undefined,
          // url: profile ? getAProfileLink(profile) : undefined,
        },
      ],
    };
  }

  return Metadata;
}

export const generateStaticParams = async (): Promise<any> => {
  // const templates = await fetchTemplates();
  // has "slug" key to populate route
  const templates = await api.marketplace.getWorkflowTemplatesForMarketplace();
  if (!templates) return [];
  return templates;
};

const Action = ({ template, profile }: any): JSX.Element => {
  return (
    <div className="flex justify-center">
      <a
        className="inline-block px-6 py-3 text-lg font-semibold text-white bg-purple-600 rounded-full shadow-lg hover:bg-pink-600  transition duration-300 ease-in-out transform hover:-translate-y-1 hover:scale-105"
        data-ph-capture-attribute-flow-template-id={template.flow_template_id}
        data-ph-capture-attribute-flow-template-name={
          template.flow_template_name
        }
        data-ph-capture-attribute-flow-template-profile-id={profile?.id}
        data-ph-capture-attribute-flow-template-profile-username={
          profile?.username
        }
        data-ph-capture-attribute-flow-template-slug={template.slug}
        href={`https://app.tryanything.xyz/templates/${template.flow_template_id}`}
      >
        Use This Template
      </a>
    </div>
  );
};

export default async function Template({
  params,
}: Props): Promise<JSX.Element> {
  console.log("params in TemplatePage", params);
  const templateResponse =
    await api.marketplace.getWorkflowTemplateBySlugForMarketplace(params.slug);

  if (!templateResponse) {
    notFound();
  }

  const template: DBFlowTemplate = templateResponse;
  console.log(
    "DBFLOWTEMPLATE in TemplatePage",
    JSON.stringify(template, null, 3),
  );

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
}
