import Image from "next/image";
import { notFound } from "next/navigation";

import { ProfileLinks } from "@/components/profileLinks";
import { TemplateGrid } from "@/components/templateGrid";
import {
  fetchProfile,
  fetchProfiles,
  fetchProfileTemplates,
  Profile,
} from "@/lib/fetchSupabase";

export const generateStaticParams = async () => {
  let profiles = await fetchProfiles();
  // has username key to populate route
  console.log("profiles in generateStaticParams", profiles);
  if (!profiles) return [];
  let goodProfiles = profiles
    .filter((profile) => profile.username !== null)
    .map((profile) => profile.username);
  return goodProfiles;
};

export default async function Profile({
  params,
}: {
  params: { username: string };
}) {
  console.log("params in ProfilePage", params);
  const profile = await fetchProfile(params.username);
  const templates = await fetchProfileTemplates(params.username);

  if (!profile || !templates) {
    //only show users that exist with templates
    notFound();
  }

  return (
    <div className="my-6 md:my-16 flex flex-col md:flex-row max-w-7xl mx-auto">
      {/* Left Column */}
      <div className="max-w-sm h-full p-6">
        <div className="avatar">
          <div className="w-24 rounded-full">
            <Image
              width={100}
              height={100}
              src={profile.avatar_url ? profile.avatar_url : ""}
              alt={profile.username ? profile.username : "user profile picture"}
            />
          </div>
        </div>
        <div className="text-3xl">{profile.full_name}</div>
        <div className="mt-2 opacity-70">@{profile.username}</div>
        <div className="mt-2">{profile.bio}</div>
        <ProfileLinks profile={profile} />
      </div>
      {/* Right Column */}
      <div className="flex flex-col p-2 md:pl-5">
        <div className="text-2xl pl-2 pb-4">Templates</div>
        <TemplateGrid templates={templates} profile={false} />
      </div>
    </div>
  );
}
