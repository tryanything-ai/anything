import React from "react";
import Logo from "@/public/icon.png";
import Image from "next/image";
import { AvatarAndUsername } from "@/components/avatarAndUsername";

export default function OgTemplates() {
  const Container = ({ children }: any) => (
    <div style={{ width: "600px", height: "400px", border: "1px solid white" }}>
      {children}
    </div>
  );

  return (
    <div className="mt-16 flex flex-col items-center gap-4">
      <div>Templates</div>
      <Container>
        <TemplateOgImage
          title={templateOgImageProps.title}
          creator={templateOgImageProps.creator}
            // profileImage={templateOgImageProps.profileImage}
          profileImage={Logo.src}
          mainTitle={templateOgImageProps.mainTitle}
          mainIcon={templateOgImageProps.mainIcon}
          nodes={templateOgImageProps.nodes}
        />
      </Container>
    </div>
  );
}

type TemplateOgImage = {
  title: string;
  creator: string;
  //   profileImage: string;
  mainTitle: string;
  mainIcon: string;
  nodes: string[];
};

const templateOgImageProps: TemplateOgImage = {
  title: "Flow Title",
  creator: "Mock Creator",
  profileImage: "",
  mainTitle: "Anything Templates",
  mainIcon: "Mock Icon",
  nodes: ["Mock Tutorial 1", "Mock Tutorial 2", "Mock Tutorial 3"],
};

// const imageLoader = ({ src, width, quality }: any) => {
//   return src;
// };

const TemplateOgImage: React.FC<TemplateOgImage> = ({
  title,
  creator,
    profileImage,
  mainTitle,
  mainIcon,
  nodes: tutorials,
}) => {
  //   console.log("Logo", Logo);
  return (
    <div className="flex flex-row w-full h-full bg-base-100">
      {/* Left */}
      <div className="w-1/2 flex flex-col p-6">
        <div className="text-xl">{mainTitle}</div>
        <div className="text-2xl">{title}</div>
        <AvatarAndUsername
          username={creator}
          avatar_url={profileImage}
          profile_name={creator}
        />
        <Image src={Logo} alt="Logo" width={50} height={50} />
      </div>
      {/* Right */}
      <div className="w-1/2 flex flex-col">Right</div>
    </div>
  );
};

// export default NotionCard;
