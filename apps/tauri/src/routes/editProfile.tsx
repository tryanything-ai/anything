import { PageHeader } from "../components/wholePageHeader";
import { useAuthenticaionContext } from "../context/AuthenticaionProvider";
import PageLayout from "../pageLayout";
import { useNavigate } from "react-router-dom";
import { SubmitHandler, useForm } from "react-hook-form";

import {
  FaGithub,
  FaGlobe,
  FaInstagram,
  FaLinkedin,
  FaTiktok,
  FaXTwitter,
  FaYoutube,
} from "react-icons/fa6";
import { useMarketplaceContext } from "../context/MarketplaceProvider";
import { useState } from "react";

export default function EditProfile() {
  const [loading, setLoading] = useState(false);
  const { profile } = useAuthenticaionContext();
  const navigate = useNavigate();
  const { updateProfile } = useMarketplaceContext();

  type Inputs = {
    bio: string;
    twitter: string;
    youtube: string;
    instagram: string;
    tiktok: string;
    github: string;
    linkedin: string;
    website: string;
  };

  const {
    register,
    handleSubmit,
    formState: { errors, isDirty },
  } = useForm<Inputs>();

  const uploadAvatar = () => {};

  const onSubmit: SubmitHandler<Inputs> = async (data, event) => {
    try {
      setLoading(true);
      console.log(data);

      let result = await updateProfile(profile.id, data);
      console.log(result);
      if (!result) {
        throw new Error("Failed to update profile");
      }
    } catch (error) {
      console.log(error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <PageLayout requireAuth>
      <PageHeader
        title="Edit Profile"
        buttonLabel="View Public Profile"
        callback={() => navigate("/" + profile.username)}
      />
      {profile ? (
        <div className="flex flex-col h-full w-full gap-5 py-16">
          {/* Profile */}
          <div className="avatar pl-10">
            <div className="w-56 h-56 rounded-full">
              <img
                // width={20}
                // height={20}
                src={profile.avatar_url ? profile.avatar_url : ""}
                alt={profile.username ? profile.username : ""}
              />
            </div>
          </div>
          <button
            className="btn btn-primary w-56 ml-10"
            onClick={() => {
              uploadAvatar();
            }}
          >
            Upload Avatar
          </button>
          {/* Form */}
          <form
            onSubmit={handleSubmit(onSubmit)}
            className="flex flex-col gap-5"
          >
            <div className="flex flex-row">
              <div className="flex mb-1 h-fulljustify-center items-center pr-4">
                Bio
              </div>
              <textarea
                // type="text"
                placeholder="Type here"
                className="w-96 textarea textarea-bordered textarea-md"
                defaultValue={profile.bio}
                {...register("bio")}
              />
            </div>

            {/* Social */}
            <div className="flex flex-row">
              <div className="flex mb-1 h-full justify-center items-center pr-4">
                <FaXTwitter />
              </div>
              <input
                type="text"
                placeholder="Type here"
                className="input input-bordered input-md w-96 "
                defaultValue={profile.twitter}
                {...register("twitter", {
                  pattern: /^(http|https):\/\/[^ "]+$/,
                })}
              />
              {errors.twitter?.type === "pattern" && (
                <p className="flex mb-1 h-full justify-center items-center pl-4">
                  Not a valid url
                </p>
              )}
            </div>
            <div className="flex flex-row">
              <div className="flex mb-1 h-full justify-center items-center pr-4">
                <FaLinkedin />
              </div>
              <input
                type="text"
                placeholder="Type here"
                className="input input-bordered input-md w-96 "
                defaultValue={profile.linkedin}
                {...register("linkedin", {
                  pattern: /^(http|https):\/\/[^ "]+$/,
                })}
              />
              {errors.linkedin?.type === "pattern" && (
                <p className="flex mb-1 h-full justify-center items-center pl-4">
                  Not a valid url
                </p>
              )}
            </div>
            <div className="flex flex-row">
              <div className="flex mb-1 h-full justify-center items-center pr-4">
                <FaGithub />
              </div>
              <input
                type="text"
                placeholder="Type here"
                className="input input-bordered input-md w-96 "
                defaultValue={profile.github}
                {...register("github", {
                  pattern: /^(http|https):\/\/[^ "]+$/,
                })}
              />
              {errors.github?.type === "pattern" && (
                <p className="flex mb-1 h-full justify-center items-center pl-4">
                  Not a valid url
                </p>
              )}
            </div>
            <div className="flex flex-row">
              <div className="flex mb-1 h-full justify-center items-center pr-4">
                <FaGlobe />
              </div>
              <input
                type="text"
                placeholder="Type here"
                className="input input-bordered input-md w-96 "
                defaultValue={profile.website}
                {...register("website", {
                  pattern: /^(http|https):\/\/[^ "]+$/,
                })}
              />
              {errors.website?.type === "pattern" && (
                <p className="flex mb-1 h-full justify-center items-center pl-4">
                  Not a valid url
                </p>
              )}
            </div>
            <div className="flex flex-row">
              <div className="flex mb-1 h-full justify-center items-center pr-4">
                <FaInstagram />
              </div>
              <input
                type="text"
                placeholder="Type here"
                className="input input-bordered input-md w-96 "
                defaultValue={profile.instagram}
                {...register("instagram", {
                  pattern: /^(http|https):\/\/[^ "]+$/,
                })}
              />
              {errors.instagram?.type === "pattern" && (
                <p className="flex mb-1 h-full justify-center items-center pl-4">
                  Not a valid url
                </p>
              )}
            </div>
            <div className="flex flex-row">
              <div className="flex mb-1 h-full justify-center items-center pr-4">
                <FaTiktok />
              </div>
              <input
                type="text"
                placeholder="Type here"
                className="input input-bordered input-md w-96 "
                defaultValue={profile.tiktok}
                {...register("tiktok", {
                  pattern: /^(http|https):\/\/[^ "]+$/,
                })}
              />
              {errors.tiktok?.type === "pattern" && (
                <p className="flex mb-1 h-full justify-center items-center pl-4">
                  Not a valid url
                </p>
              )}
            </div>
            <div className="flex flex-row">
              <div className="flex mb-1 h-full justify-center items-center pr-4">
                <FaYoutube />
              </div>
              <input
                type="text"
                placeholder="Type here"
                className="input input-bordered input-md w-96 "
                defaultValue={profile.youtube}
                {...register("youtube", {
                  pattern: /^(http|https):\/\/[^ "]+$/,
                })}
              />
              {errors.youtube?.type === "pattern" && (
                <p className="flex mb-1 h-full justify-center items-center pl-4">
                  Not a valid url
                </p>
              )}
            </div>
            {loading ? (
              "loading"
            ) : (
              <button
                className="ml-8 mt-2 btn btn-primary mb-10 w-96"
                type="submit"
                disabled={!isDirty}
              >
                Save
              </button>
            )}
          </form>
        </div>
      ) : (
        "loading"
      )}
    </PageLayout>
  );
}
