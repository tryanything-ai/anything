import { TemplateView } from "@anything/ui";
import { getAProfileLink } from "@anything/utils";
import type { Metadata, ResolvingMetadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";

import { Avatar } from "@/components/avatar";
import {
  fetchProfile,
  fetchTemplateBySlug,
  fetchTemplates,
  Profile,
} from "@/lib/fetchSupabase";

type Props = {
  params: { slug: string };
  searchParams: { [key: string]: string | string[] | undefined };
};

export async function generateMetadata(
  { params }: Props,
  parent: ResolvingMetadata
): Promise<Metadata> {
  let Metadata: Metadata = {};

  // fetch data
  const templateResponse = await fetchTemplateBySlug(params.slug);

  if (templateResponse) {
    let template = templateResponse[0];

    let profile: Profile | undefined = template?.profiles?.username
      ? await fetchProfile(template.profiles.username)
      : undefined;

    // let flow = flowJsonFromBigFlow(template) as FlowTemplate;

    // optionally access and extend (rather than replace) parent metadata
    // const previousImages = (await parent).openGraph?.images || []
    Metadata = {
      title: template.flow_template_name,
      description: template?.flow_template_description,
      // openGraph: {
      //   images: [profile?.avatar_url],
      // },
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

export const generateStaticParams = async () => {
  let templates = await fetchTemplates();
  // console.log("templates in generateStaticParams", templates);
  // has "slug" key to populate route
  if (!templates) return [];
  return templates;
};

export default async function Template({ params }: Props) {
  console.log("params in TemplatePage", params);
  const templateResponse = await fetchTemplateBySlug(params.slug);

  if (!templateResponse) {
    notFound();
  }
  
  let template = templateResponse[0];
  console.log("template in TemplatePage", JSON.stringify(template, null, 3));

  let profile: Profile | undefined = template?.profiles?.username
    ? await fetchProfile(template.profiles.username)
    : undefined;

  // let flow = flowJsonFromBigFlow(template) as FlowTemplate;

  return (
    <div className="mx-4 my-6 flex max-w-4xl flex-col md:mx-auto md:my-16">
      <TemplateView
        template={template}
        profile={profile}
        Avatar={Avatar}
        Link={Link}
      />
    </div>
  );
}