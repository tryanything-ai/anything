import React from "react";
import { OgDiv } from "./baseOg";

let svg = "http://" + process.env.NEXT_PUBLIC_VERCEL_URL + "/3og.svg";
let logo = "http://" + process.env.NEXT_PUBLIC_VERCEL_URL + "/icon.png";

export type FlowTemplateOgImageProps = {
  title: string;
  username: string;
  profileName: string;
  profileImage: string;
  trigger: any;
  actions: any[];
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
    <OgDiv
      style={{
        flexDirection: "column",
        width: "100%",
        height: "100%",
      }}
    >
      <OgDiv
        style={{
          paddingLeft: "3rem",
          paddingRight: "3rem",
          paddingTop: "1.5rem",
          height: "20%",
          fontSize: "80px",
          fontWeight: "700",
          whiteSpace: "nowrap",
          overflow: "hidden",
          textOverflow: "ellipsis",
          letterSpacing: "-0.05em",
        }}
      >
        {title}
      </OgDiv>
      <OgDiv
        style={{
          flexDirection: "row",
          height: "80%",
        }}
      >
        {/* Left */}
        <OgDiv
          style={{
            flexDirection: "column",
            justifyContent: "space-between",
            width: "50%",
            padding: "3rem",
          }}
        >
          {/* Avatar */}
          <OgDiv style={{ flexDirection: "row" }}>
            <OgDiv>
              <OgDiv style={{ width: "6rem" }}>
                <img
                  style={{ borderRadius: "50%" }}
                  src={profileImage}
                  alt={profileName}
                />
              </OgDiv>
            </OgDiv>
            <OgDiv
              style={{
                display: "flex",
                flexDirection: "column",
                justifyContent: "center",
                paddingLeft: "2rem",
              }}
            >
              <OgDiv style={{ textOverflow: "ellipsis", fontSize: "50px" }}>
                {profileName}
              </OgDiv>
            </OgDiv>
          </OgDiv>
          {/* End Avatar */}
          <OgDiv style={{ fontSize: "50px" }}>Anything Templates</OgDiv>
        </OgDiv>
        {/* Right */}
        <OgDiv
          style={{
            flexDirection: "column",
            width: "50%",
          }}
        >
          <img
            src={svg}
            alt="Magic 3og"
            style={{
              maxWidth: "100%",
              maxHeight: "100%",
              transform: "translateY(20px)",
            }}
          />
        </OgDiv>
      </OgDiv>
    </OgDiv>
  );
};

export const FlowTemplateOgImage2: React.FC<FlowTemplateOgImageProps> = ({
  title,
  username,
  profileName,
  profileImage,
  trigger,
  actions,
}) => {
  return (
    <div
      style={{
        // backgroundColor: "var(--base-100)",
        display: "flex",
        // height: "100%",
        // width: "100%",
        // flexDirection: "column",
        // padding: "1.5rem",
      }}
    >
      {/* Top */}
      <div
        style={{
          //   marginBottom: "1.25rem",
          display: "flex",
          //   flexDirection: "row",
        }}
      >
        <div
          style={{
            display: "flex",
            // marginBottom: "0.75rem",
            // width: "100%",
            // overflow: "hidden",
            // fontSize: "5rem",
            // fontWeight: "600",
          }}
        >
          {title}
        </div>
      </div>

      {/* Left */}
      <div
        style={{
          display: "flex",
          //   height: "100%",
          //   flexDirection: "row"
        }}
      >
        <div
          style={{
            display: "flex",
            // width: "50%",
            // flexDirection: "column",
            // justifyContent: "space-between",
          }}
        >
          <div style={{ display: "flex" }}>
            {/* <AvatarAndUsername
              AvatarComponent={() =>
                Avatar({ avatar_url: profileImage, profile_name: profileName })
              }
              Link={Link}
              link={false}
              profile_name={profileName}
              username={username}
            /> */}
          </div>
          <div
            style={{
              display: "flex",
              //   fontSize: "1.25rem"
            }}
          >
            Anything Templates
          </div>
        </div>
        {/* Right */}
        <div
          style={{
            display: "flex",
            //   width: "50%",
            //   flexDirection: "column"
          }}
        >
          <div
            style={{
              display: "flex",
              //   marginBottom: "1rem"
            }}
          >
            <div
              style={{
                display: "flex",
                //   fontSize: "1.5rem"
              }}
            >
              When:
            </div>
            {/* <BaseNodeWeb node={trigger} /> */}
          </div>
          <div>
            <div
              style={{
                display: "flex",
                //   fontSize: "1.5rem"
              }}
            >
              Do:
            </div>
            {/* <div style={{marginTop: '0.5rem', display: 'flex', flexDirection: 'row', gap: '0.5rem'}}>
              {actions.map((action, index) => {
                return (
                  <BaseNodeIcon icon={action.icon} key={action.node_label} />
                );
              })}
            </div> */}
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

// export const UserProfileOgImage: React.FC<ProfileOgImageProps> = ({
//   fullName,
//   username,
//   profileImage,
// }) => {
//   return (
//     <div className="bg-base-100 flex  h-full w-full flex-row p-6">
//       <div className="flex w-2/3 flex-col justify-between">
//         <div className="text-xl">Anything Templates</div>
//         <div>
//           <div className="text-4xl font-semibold">{fullName}</div>
//           <div className="text-2xl opacity-70">@{username}</div>
//         </div>
//         <div className="mr-2">
//           <Image src={Logo} alt="Logo" width={50} height={50} />
//         </div>
//       </div>
//       {/* Right */}
//       <div className="flex h-full w-1/3 flex-col justify-center">
//         <div className="avatar">
//           <div className="h-42 w-42 rounded-full">
//             <Image src={Logo} alt={fullName} />
//           </div>
//         </div>
//       </div>
//     </div>
//   );
// };
