import type { Profile } from "utils";
import { formatUrl, hasLinks } from "utils";
import { ComponentType } from "react";
import {
  FaGithub,
  FaGlobe,
  FaInstagram,
  FaLinkedin,
  FaTiktok,
  FaXTwitter,
  FaYoutube,
} from "react-icons/fa6";

export const ProfileLinks = ({
  profile,
  Link,
}: {
  profile: Profile;
  Link: ComponentType<any>;
}) => {
  if (hasLinks(profile)) {
    return (
      <div className="mt-6">
        <div className="mb-1 opacity-70">Links</div>
        {profile.twitter && (
          <div className="flex h-6 flex-row">
            <FaXTwitter className="mr-2 h-6 w-3" />
            <Link
              href={profile.twitter}
              to={profile.twitter}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm"
            >
              {formatUrl(profile.twitter)}
            </Link>
          </div>
        )}
        {profile.linkedin && (
          <div className="flex h-6 flex-row">
            <FaLinkedin className="mr-2 h-6 w-3" />
            <Link
              href={profile.linkedin}
              to={profile.linkedin}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm"
            >
              {formatUrl(profile.linkedin)}
            </Link>
          </div>
        )}
        {profile.github && (
          <div className="flex h-6 flex-row">
            <FaGithub className="mr-2 h-6 w-3" />
            <Link
              href={profile.github}
              to={profile.github}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm"
            >
              {formatUrl(profile.github)}
            </Link>
          </div>
        )}
        {profile.website && (
          <div className="flex h-6 flex-row">
            <FaGlobe className="mr-2 h-6 w-3" />
            <Link
              href={profile.website}
              to={profile.website}  
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm"
            >
              {formatUrl(profile.website)}
            </Link>
          </div>
        )}
        {profile.instagram && (
          <div className="flex h-6 flex-row">
            <FaInstagram className="mr-2 h-6 w-3" />
            <Link
              href={profile.instagram}
              to={profile.instagram}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm"
            >
              {formatUrl(profile.instagram)}
            </Link>
          </div>
        )}
        {profile.tiktok && (
          <div className="flex h-6 flex-row">
            <FaTiktok className="mr-2 h-6 w-3" />
            <Link
              href={profile.tiktok}
              to={profile.tiktok}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm"
            >
              {formatUrl(profile.tiktok)}
            </Link>
          </div>
        )}
        {profile.youtube && (
          <div className="flex h-6 flex-row">
            <FaYoutube className="mr-2 h-6 w-3" />
            <Link
              href={profile.youtube}
              to={profile.youtube}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm"
            >
              {formatUrl(profile.youtube)}
            </Link>
          </div>
        )}
      </div>
    );
  } else {
    return null;
  }
};
