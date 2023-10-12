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
    <div className="my-6 md:my-16 flex flex-col md:flex-row max-w-7xl mx-auto">
      {/* Left Column */}
      <div className="max-w-sm h-full p-6">
        <div className="avatar">
          <div className="w-24 rounded-full">
            <Image
              width={100}
              height={100}
              src={profile && profile.avatar_url ? profile.avatar_url : ""}
              alt={
                profile && profile.username
                  ? profile.username
                  : "user profile picture"
              }
            />
          </div>
        </div>
        {/* <div className="text-3xl">{profile.full_name}</div>
        <div className="mt-2 opacity-70">@{profile.username}</div>
        <div className="mt-2">{profile.bio}</div> */}
        {profile && <ProfileLinks profile={profile} />}
      </div>
      {/* Right Column */}
      <div className="flex flex-col p-2 md:pl-5">
        <div className="text-2xl pl-2 pb-4">Templates</div>
        {/* <TemplateGrid templates={templates} profile={false} /> */}
      </div>
    </div>
  );
}
