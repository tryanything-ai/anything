import Image from "next/image";

export const AvatarAndUsername = ({ avatar_url, profile_name }: { avatar_url: string, profile_name: string }) => {
  return (
    <div className="flex flex-row">
      <div className="avatar">
        <div className="w-10 rounded-full">
          <Image width={100} height={100} src={avatar_url} alt={profile_name} />
        </div>
      </div>
      <div className="flex flex-col pl-4 justify-center">
        <div className="text-ellipsis">{profile_name}</div>
        {/* <div className="opacity-70">20 templates</div> */}
      </div>
    </div>
  );
};
