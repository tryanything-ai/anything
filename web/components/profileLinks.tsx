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
import { formatUrl, hasLinks} from "@/utils/frontEndUtils";
import { Profile } from "@/types/supabase.types";

export const ProfileLinks = ({ profile }: { profile: Profile }) => {
    if (hasLinks(profile)) {
        return (
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
        );
    } else {
        return null; 
    }
}