import {
  Profile,
  fetchProfile,
  fetchTemplateBySlug,
  fetchTemplates,
} from "@/lib/fetchSupabase";
import { notFound } from "next/navigation";
import Image from "next/image";
// import { TemplateGrid } from "@/components/templateGrid";
import { ProfileLinks } from "@/components/profileLinks";
import { AvatarAndUsername } from "@/components/avatarAndUsername";
import { Button } from "@/components/ui/Button";
import { Tags } from "@/components/tags";

export const generateStaticParams = async () => {
  let templates = await fetchTemplates();
  // console.log("templates in generateStaticParams", templates);
  // has "slug" key to populate route
  return templates;
};

export default async function Template({
  params,
}: {
  params: { slug: string };
}) {
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
          />
        </div>
        {/* Right */}
        <div>
          <Button>Use this template</Button>
        </div>
      </div>
      <div className="font-semibold mt-8 mb-2">About this template</div>
      <div className="">{template.flow_template_description}</div>
      <div className="font-semibold mt-8 mb-2">Tags</div>
      <Tags tags={template.tags} />
      <div className="font-semibold mt-8 mb-2">Trigger</div>
      <div>TRIGGER</div>
      <div className="font-semibold mt-8 mb-2">Actions</div>
      <div>ACTIONS</div>
    </div>
  );
}
