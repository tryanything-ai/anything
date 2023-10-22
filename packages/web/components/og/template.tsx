import Image from "next/image";
import React from "react";

import { AvatarAndUsername } from "@/components/avatarAndUsername";
import BaseNodeIcon from "@/components/baseNodeIcons";
import { BaseNodeWeb } from "@/components/baseNodeWeb";
import Logo from "@/public/icon.png";
import { Node } from "@/types/flow";

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
    <div className="flex flex-col w-full h-full bg-base-100 p-6">
      {/* Top */}
      <div className="flex flex-row mb-5">
        <div className="text-5xl mb-3 font-semibold w-full overflow-ellipsis">
          {title}
        </div>
      </div>

      {/* Left */}
      <div className="flex flex-row h-full">
        <div className="w-1/2 flex flex-col justify-between">
          <div>
            <AvatarAndUsername
              username={username}
              avatar_url={profileImage}
              profile_name={profileName}
            />
          </div>
          <div className="text-xl">Anything Templates</div>
        </div>
        {/* Right */}
        <div className="w-1/2 flex flex-col">
          <div className="mb-4">
            <div className="text-2xl">When:</div>
            <BaseNodeWeb node={trigger} />
          </div>
          <div>
            <div className="text-2xl">Do:</div>
            <div className="flex flex-row gap-2 mt-2">
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
    <div className="flex flex-row  w-full h-full bg-base-100 p-6">
      <div className="w-2/3 flex flex-col justify-between">
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
      <div className="w-1/3 flex flex-col justify-center h-full">
        <div className="avatar">
          <div className="rounded-full h-42 w-42">
            <Image src={Logo} alt={fullName} />
          </div>
        </div>
      </div>
    </div>
  );
};
