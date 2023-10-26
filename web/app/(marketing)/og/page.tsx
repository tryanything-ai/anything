import React from "react";
import Logo from "@/public/icon.png";
import { MockNewFlows } from "@/types/flow";

import {
  FlowTemplateOgImage,
  UserProfileOgImage,
  FlowTemplateOgImageProps,
  ProfileOgImageProps,
} from "@/components/og/template";

const mockFlowProps: FlowTemplateOgImageProps = {
  title: "Flow Title",
  username: "mockCreator",
  profileName: "Mock Creator",
  profileImage: "",
  trigger: MockNewFlows[0].trigger,
  actions: MockNewFlows[0].actions,
};

const mockProfileProps: ProfileOgImageProps = {
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
