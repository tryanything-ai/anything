import { BaseNodeIcon } from "@repo/ui/components/baseNodeIcons";
// import { BaseNodeWeb } from "@repo/ui/components/baseNodeWeb";
import { AvatarAndUsername } from "@repo/ui/components/avatarAndUsername";
import { Node } from "@/types/flow";
import Image from "next/image";
import Link from "next/link";
import React from "react";
import Logo from "@/public/icon.png";
import { Avatar } from "@/components/avatar";

export type FlowTemplateOgImageProps = {
  title: string;
  username: string;
  profileName: string;
  profileImage: string;
  trigger: Node;
  actions: Node[];
};

export const FlowTemplateOgImage: React.FC<FlowTemplateOgImageProps> = ({
  title,
  username,
  profileName,
  profileImage,
  trigger,
  actions,
}) => {
  return (
    <div className="bg-base-100 flex h-full w-full flex-col p-6">
      {/* Top */}
      <div className="mb-5 flex flex-row">
        <div className="mb-3 w-full overflow-ellipsis text-5xl font-semibold">
          {title}
        </div>
      </div>

      {/* Left */}
      <div className="flex h-full flex-row">
        <div className="flex w-1/2 flex-col justify-between">
          <div>
            <AvatarAndUsername
              AvatarComponent={() =>
                Avatar({ avatar_url: profileImage, profile_name: profileName })
              }
              Link={Link as any}
              link={false}
              profile_name={profileName}
              username={username}
            />
          </div>
          <div className="text-xl">Anything Templates</div>
        </div>
        {/* Right */}
        <div className="flex w-1/2 flex-col">
          <div className="mb-4">
            <div className="text-2xl">When:</div>
            {/* <BaseNodeWeb node={trigger} /> */}
          </div>
          <div>
            <div className="text-2xl">Do:</div>
            <div className="mt-2 flex flex-row gap-2">
              {actions.map((action, index) => {
                return (
                  <BaseNodeIcon icon={action.icon} key={action.node_label} />
                );
              })}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export type ProfileOgImageProps = {
  fullName: string;
  username: string;
  profileImage: string;
};

export const UserProfileOgImage: React.FC<ProfileOgImageProps> = ({
  fullName,
  username,
  profileImage,
}) => {
  return (
    <div className="bg-base-100 flex  h-full w-full flex-row p-6">
      <div className="flex w-2/3 flex-col justify-between">
        <div className="text-xl">Anything Templates</div>
        <div>
          <div className="text-4xl font-semibold">{fullName}</div>
          <div className="text-2xl opacity-70">@{username}</div>
        </div>
        <div className="mr-2">
          <Image src={Logo} alt="Logo" width={50} height={50} />
        </div>
      </div>
      {/* Right */}
      <div className="flex h-full w-1/3 flex-col justify-center">
        <div className="avatar">
          <div className="h-42 w-42 rounded-full">
            <Image src={Logo} alt={fullName} />
          </div>
        </div>
      </div>
    </div>
  );
};
