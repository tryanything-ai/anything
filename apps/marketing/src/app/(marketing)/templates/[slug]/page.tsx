import { TemplateView } from "@repo/ui/components/TemplateView";
import {
  fetchProfile,
  fetchTemplateBySlug,
  fetchTemplates,
} from "@/lib/supabase/fetchSupabase";
import { getAProfileLink } from "@repo/ui/helpers/helpers";
import type { Metadata, ResolvingMetadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import { Avatar } from "@/components/avatar";

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
  const templateResponse = await fetchTemplateBySlug(params.slug);

  if (templateResponse) {
    const template = templateResponse[0];

    const profile: any | undefined = template?.profiles?.username
      ? await fetchProfile(template.profiles.username)
      : undefined;

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
          name: profile?.full_name || undefined,
          url: profile ? getAProfileLink(profile) : undefined,
        },
      ],
    };
  }

  return Metadata;
}

export const generateStaticParams = async (): Promise<any> => {
  const templates = await fetchTemplates();
  // has "slug" key to populate route
  if (!templates) return [];
  return templates;
};

const Action = ({ template, profile }: any): JSX.Element => {
  return (
    <div className="flex flex-col gap-3 md:flex-row">
      <Link
        className="btn btn-sm btn-primary md:btn-md"
        data-ph-capture-attribute-flow-template-id={template.flow_template_id}
        data-ph-capture-attribute-flow-template-name={
          template.flow_template_name
        }
        data-ph-capture-attribute-flow-template-profile-id={profile?.id}
        data-ph-capture-attribute-flow-template-profile-username={
          profile?.username
        }
        data-ph-capture-attribute-flow-template-slug={template.slug}
        href="/downloads"
      >
        Download Anything
      </Link>
      <a
        className="btn btn-sm btn-primary md:btn-md"
        data-ph-capture-attribute-flow-template-id={template.flow_template_id}
        data-ph-capture-attribute-flow-template-name={
          template.flow_template_name
        }
        data-ph-capture-attribute-flow-template-profile-id={profile?.id}
        data-ph-capture-attribute-flow-template-profile-username={
          profile?.username
        }
        data-ph-capture-attribute-flow-template-slug={template.slug}
        href={`anything://templates/${template.slug}`}
      >
        Open in App
      </a>
    </div>
  );
};

export default async function Template({
  params,
}: Props): Promise<JSX.Element> {
  console.log("params in TemplatePage", params);
  const templateResponse = await fetchTemplateBySlug(params.slug);

  if (!templateResponse) {
    notFound();
  }

  const template = templateResponse[0];
  console.log("template in TemplatePage", JSON.stringify(template, null, 3));

  const profile: any | undefined = template?.profiles?.username
    ? await fetchProfile(template.profiles.username)
    : undefined;

  return (
    <div className="mx-4 my-6 flex max-w-4xl flex-col md:mx-auto md:my-16">
      <TemplateView
        ActionComponent={Action}
        Avatar={Avatar}
        Link={Link}
        profile={profile}
        template={template}
      />
    </div>
  );
}
