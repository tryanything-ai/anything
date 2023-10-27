import { Link } from "react-router-dom";

import { useAuthenticaionContext } from "../context/AuthenticaionProvider";
import PageLayout from "../pageLayout";

export default function Profile() {
  const { profile } = useAuthenticaionContext();

  const uploadAvatar = () => {};

  if (!profile)
    return (
      <PageLayout>
        <Link to="/login" className="btn btn-primary m-1 ml-4">
          Login
        </Link>
      </PageLayout>
    );

  return (
    <PageLayout>
      <div className="flex flex-row h-full w-full m-10">
        {/* Profile */}
        Profile
        <div className="avatar">
          <div className="w-100 rounded-full">
            <img
              width={100}
              height={100}
              src={profile.avatar_url ? profile.avatar_url : ""}
              alt={profile.username ? profile.username : ""}
            />
          </div>
        </div>
        <button
          className="btn btn-primary m-1 ml-4"
          onClick={() => {
            uploadAvatar();
          }}
        >
          Upload Avatar
        </button>
      </div>
    </PageLayout>
  );
}
