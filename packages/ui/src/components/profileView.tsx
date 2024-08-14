// import { BigFlow, Profile } from "../types/flow";

import type { CommonProps } from "./commonTypes";
import { ProfileLinks } from "./profileLinks";

import { TemplateGrid } from "./templateGrid";

interface ProfileViewProps extends CommonProps {
  templates: any;
  profile: any;
}

export const ProfileView = ({
  profile,
  templates,
  Link,
  Avatar,
}: ProfileViewProps) => {
  return (
    <div className="flex max-w-7xl flex-col md:flex-row w-full">
      {/* Left Column */}
      <div className="h-full max-w-sm p-6">
        <div className="avatar">
          <div className="w-24 rounded-full">
            <Avatar
              avatar_url={
                profile.avatar_url
                  ? profile.avatar_url
                  : "https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/public/mocks/botttsNeutral-1698715092376.png"
              }
              profile_name={profile.full_name ? profile.full_name : ""}
              // width={100}
            />
            {/* <Avatar
                width={100} 
                height={100}
                src={profile.avatar_url ? profile.avatar_url : ""}
                alt={profile.username ? profile.username : "user profile picture"}
              /> */}
          </div>
        </div>
        <div className="text-3xl">{profile.full_name}</div>
        <div className="mt-2 opacity-70">@{profile.username}</div>
        <div className="mt-2">{profile.bio}</div>
        <ProfileLinks profile={profile} Link={Link} />
      </div>
      {/* Right Column */}
      <div className="flex flex-col p-2 md:pl-5">
        <div className="pb-4 pl-2 text-2xl">Templates</div>
        <TemplateGrid
          LinkComponent={Link}
          AvatarComponent={Avatar}
          templates={templates}
          profile={false}
        />
      </div>
    </div>
  );
};
