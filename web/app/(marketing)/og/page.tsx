import React from "react";
import Logo from "@/public/icon.png";
import Image from "next/image";
import { AvatarAndUsername } from "@/components/avatarAndUsername";
import { BaseNodeWeb } from "@/components/baseNodeWeb";
import { MockNewFlows, Node } from "@/types/flow";
import BaseNodeIcon from "@/components/baseNodeIcons";

const mockFlowProps: FlowTemplateOgImage = {
  title: "Flow Title",
  username: "mockCreator",
  profileName: "Mock Creator",
  profileImage: "",
  trigger: MockNewFlows[0].trigger,
  actions: MockNewFlows[0].actions,
};

const mockProfileProps: ProfileOgImage = {
  fullName: "Mock Full Name",
  username: "username_",
  profileImage: "",
};

const Normal = ({ children }: any) => (
  <div style={{ width: "600px", height: "400px", border: "1px solid white" }}>
    {children}
  </div>
);

const SquareContainer = ({ children }: any) => (
  <div style={{ width: "300px", height: "300px", border: "1px solid white" }}>
    {children}
  </div>
);

const TallContainer = ({ children }: any) => (
  <div style={{ width: "300px", height: "600px", border: "1px solid white" }}>
    {children}
  </div>
);

const MulitContainer = ({ children }: any) => {
  return (
    <>
      <Normal>{children}</Normal>
      <SquareContainer>{children}</SquareContainer>
      <TallContainer>{children}</TallContainer>
    </>
  );
};

export default function OgTemplates() {
  return (
    <div className="mt-16 flex flex-col items-center gap-4">
      <div className="text-5xl">User Profiles</div>
      <MulitContainer>
        <UserProfileOgImage
          username={mockProfileProps.username}
          fullName={mockProfileProps.fullName}
          profileImage={Logo.src}
        />
      </MulitContainer>
      <div className="text-5xl">Flow Templates</div>
      <MulitContainer>
        <FlowTemplateOgImage
          title={mockFlowProps.title}
          profileName={mockFlowProps.profileName}
          username={mockFlowProps.username}
          profileImage={Logo.src}
          trigger={mockFlowProps.trigger}
          actions={mockFlowProps.actions}
        />
      </MulitContainer>
    </div>
  );
}

type FlowTemplateOgImage = {
  title: string;
  username: string;
  profileName: string;
  profileImage: string;
  trigger: Node;
  actions: Node[];
};

export const FlowTemplateOgImage: React.FC<FlowTemplateOgImage> = ({
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
        {/* <div className="mr-2">
          <Image src={Logo} alt="Logo" width={50} height={50} />
        </div> */}
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

type ProfileOgImage = {
  fullName: string;
  username: string;
  profileImage: string;
};

export const UserProfileOgImage: React.FC<ProfileOgImage> = ({
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
