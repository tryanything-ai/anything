import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import { ProfileView } from "ui";
import PageLayout from "../pageLayout";
import { useMarketplaceContext } from "../context/MarketplaceProvider";
import type { Profile, BigFlow } from "utils";
import { Avatar } from "../components/avatar";

const Profile = ({}) => {
  const { username } = useParams<{ username: string }>();
  const { fetchProfile, fetchProfileTemplates } = useMarketplaceContext();

  const [profile, setProfile] = useState<Profile>();
  const [templates, setTemplates] = useState<BigFlow>();

  const fetchAll = async () => {
    let profile = await fetchProfile(username);
    console.log("profile", profile);
    if (profile) {
      setProfile(profile);
    }
    let templates = await fetchProfileTemplates(username);
    console.log("templates", templates);
    if (templates) {
      setTemplates(templates);
    }
  };

  useEffect(() => {
    if (username) {
      fetchAll();
    }
  }, [username]);

  return (
    <PageLayout>
      {profile && templates ? (
        <ProfileView
          profile={profile}
          templates={templates}
          Link={Link}
          Avatar={Avatar}
        />
      ) : (
        "Loading"
      )}
    </PageLayout>
  );
};

export default Profile;
