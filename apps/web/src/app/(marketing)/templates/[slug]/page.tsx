import { TemplateView } from "ui";
import {
  BigFlow,
  fetchProfile,
  fetchTemplateBySlug,
  fetchTemplates,
  getAProfileLink,
  Profile,
} from "utils";
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
  parent: ResolvingMetadata
): Promise<Metadata> {
  let Metadata: Metadata = {};

  // fetch data
  const templateResponse = await fetchTemplateBySlug(params.slug);

  if (templateResponse) {
    const template = templateResponse[0];

    const profile: Profile | undefined = template?.profiles?.username
      ? await fetchProfile(template.profiles.username)
      : undefined;

    // let flow = flowJsonFromBigFlow(template) as FlowTemplate;

    // optionally access and extend (rather than replace) parent metadata
    // const previousImages = (await parent).openGraph?.images || []
    Metadata = {
      title: template.flow_template_name,
      description: template?.flow_template_description,
      openGraph: {
        description: template?.flow_template_description ?? undefined,
        // images: [profile?.avatar_url],
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

export const generateStaticParams = async (): Promise<BigFlow> => {
  const templates = await fetchTemplates();
  // console.log("templates in generateStaticParams", templates);
  // has "slug" key to populate route
  if (!templates) return [];
  return templates;
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

  const profile: Profile | undefined = template?.profiles?.username
    ? await fetchProfile(template.profiles.username)
    : undefined;

  // let flow = flowJsonFromBigFlow(template) as FlowTemplate;
  function Action({ slug }): JSX.Element {
    return (
      <div className="flex flex-col gap-3 md:flex-row">
        <div className="btn btn-sm btn-primary md:btn-md">
          Download Anything
        </div>
        <a
          className="btn btn-sm btn-primary md:btn-md"
          href={`anything://templates/${slug}`}
        >
          Open in App
        </a>
      </div>
    );
  }

  return (
    <div className="mx-4 my-6 flex max-w-4xl flex-col md:mx-auto md:my-16">
      <TemplateView
        ActionComponent={() => <Action slug={template.slug} />}
        Avatar={Avatar}
        Link={Link}
        profile={profile}
        template={template}
      />
    </div>
  );
}
