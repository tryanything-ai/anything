import React, { ReactNode } from "react";
import { ImageResponse } from "next/og";

const this_url = "http://" + process.env.VERCEL_PROJECT_PRODUCTION_URL;

// Route segment config
export const runtime = "edge";

// Image metadata
export const alt = "Anything Og Template";

export const size = {
  width: 1200,
  height: 628,
};

export const contentType = "image/png";

//always need display: flex on all components for "sartori" and open image
// https://github.com/vercel/satori#css
//https://github.com/vercel/next.js/issues/48238
export const OgDiv: React.FC<{
  children: ReactNode;
  style?: React.CSSProperties;
}> = ({ children, style }) => {
  return (
    <div className="font-display" style={{ display: "flex", ...style }}>
      {children}
    </div>
  );
};

// Image generation
export default async function Image({ params }: { params: { slug: string } }) {
  console.log(
    "params in TemplatePageOgImage Generation",
    JSON.stringify(params),
  );

  const boldFontData = await fetch(
    this_url + "/fonts/DMSans-SemiBold.ttf",
  ).then((res) => res.arrayBuffer());

  return new ImageResponse(
    (
      <div
        style={{
          height: "100%",
          width: "100%",
          display: "flex",
          fontFamily: "Dm_Sans",
          backgroundColor: "black",
        }}
      >
        Template Og Image
      </div>
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
