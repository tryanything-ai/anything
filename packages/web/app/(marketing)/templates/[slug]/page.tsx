import type { Metadata, ResolvingMetadata } from "next";
import { notFound } from "next/navigation";

import { AvatarAndUsername } from "@/components/avatarAndUsername";
import { BaseNodeWeb } from "@/components/baseNodeWeb";
import Deeplink from "@/components/deepLink";
import { ProfileLinks } from "@/components/profileLinks";
import { Tags } from "@/components/tags";
import { Button } from "@/components/ui/Button";
import {
  fetchProfile,
  fetchTemplateBySlug,
  fetchTemplates,
  Profile,
} from "@/lib/fetchSupabase";
import { FlowTemplate } from "@/types/flow";
import { flowJsonFromBigFLow, getAProfileLink } from "@/utils/frontEndUtils";

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

    let flow = flowJsonFromBigFLow(template) as FlowTemplate;

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

  let flow = flowJsonFromBigFLow(template) as FlowTemplate;

  return (
    <div className="my-6 mx-4 md:my-16 flex flex-col max-w-4xl md:mx-auto">
      <div className="text-3xl md:text-5xl font-semibold mb-6 min-h-16  ">
        {template.flow_template_name}
      </div>
      <div className="flex flex-row justify-between">
        {/* Left */}
        <div>
          <AvatarAndUsername
            profile_name={profile?.full_name ? profile.full_name : ""}
            avatar_url={profile?.avatar_url ? profile.avatar_url : ""}
            username={profile?.username ? profile.username : ""}
          />
        </div>
        {/* Right */}
        <div>
          <Button>
            {/* <Deeplink href="anything://templateid">Open in App </Deeplink> */}
            <a href={`anything://templateid`}>Open in App</a>
          </Button>
        </div>
      </div>
      <div className="font-semibold mt-8 mb-2">About this template</div>
      <div className="">{template.flow_template_description}</div>

      <div className="font-semibold mt-8 mb-2">Trigger</div>
      <div>
        <BaseNodeWeb node={flow.trigger} />
      </div>
      <div className="font-semibold mt-8 mb-2">Actions</div>
      <div>
        {flow.actions.map((action, index) => {
          return <BaseNodeWeb node={action} key={action.node_label} />;
        })}
      </div>
      <div className="font-semibold mt-8 mb-2">Tags</div>
      <Tags tags={template.tags} />
      {profile ? (
        <>
          <div className="font-semibold mt-8 mb-2">About the creator</div>
          <ProfileLinks profile={profile} />
        </>
      ) : null}
    </div>
  );
}
