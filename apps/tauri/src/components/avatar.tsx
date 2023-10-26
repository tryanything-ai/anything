export const Avatar = ({
    avatar_url,
    profile_name,
  }: {
    avatar_url: string;
    profile_name: string;
  }) => {
    return <img width={100} height={100} src={avatar_url} alt={profile_name} />;
  };