import React from "react";

export const AvatarAndUsername = ({
  profile_name,
  username,
  Link,
  AvatarComponent,
  link = true,
}: {
  profile_name: string;
  username: string;
  Link: React.ComponentType<any>;
  AvatarComponent: React.ComponentType;
  link?: boolean;
}) => {
  const Component = () => {
    return (
      <div className="flex flex-row">
        <div className="avatar">
          <div className="w-10 rounded-full">
            <AvatarComponent />
          </div>
        </div>
        <div className="flex flex-col justify-center pl-4">
          <div className="text-ellipsis">{profile_name}</div>
        </div>
      </div>
    );
  };

  if (link) {
    return (
      <Link href={"/" + username} to={"/" + username}>
        <Component />
      </Link>
    );
  } else {
    return <Component />;
  }
};
