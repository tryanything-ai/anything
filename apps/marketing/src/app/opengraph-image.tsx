import { ImageResponse } from "next/og";
import { OgDiv } from "@/components/og/baseOg";

const this_url = "http://" + process.env.NEXT_PUBLIC_HOSTED_URL;
let svg = "http://" + process.env.NEXT_PUBLIC_HOSTED_URL + "/3og.svg";

// Route segment config
export const runtime = "edge";

// Image metadata
export const alt = "Anything";

export const size = {
  width: 1200,
  height: 628,
};

export const contentType = "image/png";

// Image generation
export default async function Image({ params }: { params: any }) {
  console.log(
    "params in TemplatePageOgImage Generation",
    JSON.stringify(params),
  );

  const boldFontData = await fetch(
    this_url + "/fonts/DMSans-SemiBold.ttf",
  ).then((res) => res.arrayBuffer());

  return new ImageResponse(
    (
      <OgDiv
        style={{
          height: "100%",
          width: "100%",
          fontFamily: "Dm_Sans",
          backgroundColor: "black",
          color: "#FFFFFF",
        }}
      >
        <OgDiv
          style={{
            flexDirection: "row",
          }}
        >
          {/* Left */}
          <OgDiv
            style={{
              paddingLeft: "3rem",
              paddingTop: "5rem",
              height: "100%",
              fontSize: "150px",
              fontWeight: "700",
              whiteSpace: "nowrap",
              letterSpacing: "-0.05em",
              width: "55%",
            }}
          >
            Anything
          </OgDiv>
          {/* Right */}
          <OgDiv
            style={{
              width: "45%",
              height: "100%",
              display: "flex",
              justifyContent: "flex-end",
              flexDirection: "column",
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
    ),
    {
      ...size,
      fonts: [
        {
          name: "Dm_Sans",
          data: boldFontData,
          weight: 700,
        },
      ],
    },
  );
}
