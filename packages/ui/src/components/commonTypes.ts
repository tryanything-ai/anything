import type { JSX, ComponentType } from "react";

export type AvatarComponentProps = {
  avatar_url: string;
  profile_name: string;
};

// export type LinkComponentProps = {
//   href: string;
//   children: ReactNode;
// };

export type CommonProps = {
  Avatar: (props: AvatarComponentProps) => JSX.Element;
  //   Link: (props: LinkComponentProps) => JSX.Element;
  Link: ComponentType<any>;
};
