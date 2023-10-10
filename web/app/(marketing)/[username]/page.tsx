// Profile page
// `pages` directory
// import ProfileLayout from '@/components/post-layout'
// import { GetStaticPathsContext, GetStaticPropsContext, NextPageContext } from "next"
import { fetchProfiles, fetchProfile, Profile } from "@/lib/fetchSupabase";
import { GetStaticProps, GetStaticPaths, GetServerSideProps } from "next";
import { notFound } from "next/navigation";
import Image from "next/image";
import {
  FaTiktok,
  FaGithub,
  FaYoutube,
  FaGlobe,
  FaLinkedin,
  FaInstagram,
  FaXTwitter,
} from "react-icons/fa6";
import Link from "next/link";

const formatUrl = (url: string): string => {
  // Remove the http or https from the beginning
  const formattedUrl = url.replace(/^https?:\/\//, "");

  // If the string is longer than 30 characters, truncate and add ellipses
  if (formattedUrl.length > 32) {
    return `${formattedUrl.substring(0, 29)}...`;
  }
  return formattedUrl;
};

export const generateStaticParams = async () => {
  let profiles = await fetchProfiles();
  return profiles;
};

const hasLinks = (profile: Profile) => {
  return (
    profile.twitter ||
    profile.linkedin ||
    profile.github ||
    profile.website ||
    profile.instagram ||
    profile.tiktok ||
    profile.youtube
  );
};

export default async function Profile({ params }: any) {
  const profile = await fetchProfile(params.username);

  if (!profile) {
    notFound();
  }

  return (
    <div className="my-16 flex flex-col md:flex-row max-w-7xl mx-auto">
      {/* Left Column */}
      <div className="w-72  max-w-sm h-full">
        <div className="avatar">
          <div className="w-24 rounded-full">
            <Image
              width={100}
              height={100}
              src={profile.avatar_url ? profile.avatar_url : ""}
              alt={profile.username ? profile.username : "user profile picture"}
            />
          </div>
        </div>
        <div className="text-3xl">{profile.full_name}</div>
        <div className="mt-2 opacity-70">@{profile.username}</div>
        <div className="mt-2">{profile.bio}</div>
        {hasLinks(profile) && (
          <div className="mt-6">
            <div className="opacity-70 mb-1">Links</div>
            {profile.twitter && (
              <div className="flex flex-row h-6">
                <FaXTwitter className="h-6 w-3 mr-2" />
                <Link
                  href={profile.twitter}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm"
                >
                  {formatUrl(profile.twitter)}
                </Link>
              </div>
            )}
            {profile.linkedin && (
              <div className="flex flex-row h-6">
                <FaLinkedin className="h-6 w-3 mr-2" />
                <Link
                  href={profile.linkedin}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm"
                >
                  {formatUrl(profile.linkedin)}
                </Link>
              </div>
            )}
            {profile.github && (
              <div className="flex flex-row h-6">
                <FaGithub className="h-6 w-3 mr-2" />
                <Link
                  href={profile.github}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm"
                >
                  {formatUrl(profile.github)}
                </Link>
              </div>
            )}
            {profile.website && (
              <div className="flex flex-row h-6">
                <FaGlobe className="h-6 w-3 mr-2" />
                <Link
                  href={profile.website}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm"
                >
                  {formatUrl(profile.website)}
                </Link>
              </div>
            )}
            {profile.instagram && (
              <div className="flex flex-row h-6">
                <FaInstagram className="h-6 w-3 mr-2" />
                <Link
                  href={profile.instagram}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm"
                >
                  {formatUrl(profile.instagram)}
                </Link>
              </div>
            )}
            {profile.tiktok && (
              <div className="flex flex-row h-6">
                <FaTiktok className="h-6 w-3 mr-2" />
                <Link
                  href={profile.tiktok}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm"
                >
                  {formatUrl(profile.tiktok)}
                </Link>
              </div>
            )}
            {profile.youtube && (
              <div className="flex flex-row h-6">
                <FaYoutube className="h-6 w-3 mr-2" />
                <Link
                  href={profile.youtube}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm"
                >
                  {formatUrl(profile.youtube)}
                </Link>
              </div>
            )}
          </div>
        )}

        {/* <div>{JSON.stringify(profile, null, 3)}</div> */}
      </div>
      {/* Right Column */}
      <div>Templates</div>
    </div>
  );
}
