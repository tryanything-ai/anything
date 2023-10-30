import { PageHeader } from "../components/wholePageHeader";
import { useAuthenticationContext } from "../context/AuthenticaionProvider";
import PageLayout from "../pageLayout";
import { useNavigate } from "react-router-dom";
import { SubmitHandler, Controller, useForm } from "react-hook-form";
import EditAvatar from "../components/editAvatar";

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
  const { profile } = useAuthenticationContext();
  const navigate = useNavigate();
  const { updateProfile } = useMarketplaceContext();

  type Inputs = {
    public: boolean;
    bio: string;
    twitter: string;
    youtube: string;
    instagram: string;
    tiktok: string;
    github: string;
    linkedin: string;
    website: string;
    full_name: string;
    username: string;
  };

  const {
    register,
    handleSubmit,
    control,
    formState: { errors, isDirty, touchedFields },
  } = useForm<Inputs>();

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
  // Check if any field has been touched
  // const anyFieldTouched = Object.keys(touchedFields).length > 0;

  return (
    <PageLayout requireAuth>
      <div className="flex flex-row w-full justify-between">
        <div className="h2">Edit Profile</div>
        {profile && profile.public ? (
          <button
            className="btn btn-primary m-1 ml-4"
            onClick={() => navigate("/" + profile.username)}
          >
            View Public Profile
          </button>
        ) : null}
      </div>
      {/* <PageHeader
        title="Edit Profile"
        buttonLabel={profile.public ? "View Public Profile" : ""}
        callback={() => navigate("/" + profile.username)}
      /> */}
      {profile ? (
        <div className="flex flex-col h-full w-full gap-5 py-16">
          {/* Profile */}
          <EditAvatar
            key={profile.avatar_url}
            profile_id={profile.id}
            avatar_url={profile.avatar_url}
            size={50}
          />
          {/* Form */}
          <form
            onSubmit={handleSubmit(onSubmit)}
            className="flex flex-col gap-5 mt-5"
          >
            {/* Public */}
            <div className="flex flex-row">
              <div className="flex mb-1 h-fulljustify-center items-center pr-4">
                Public Profile
              </div>
              <input
                type="checkbox"
                defaultChecked={profile.public}
                {...register("public")}
                className="toggle toggle-success"
              />
            </div>

            {/* Bio */}
            <div className="flex flex-col">
              <div className="flex mb-1 pr-4">Bio</div>
              <textarea
                // type="text"
                placeholder="Type here"
                className="w-96 h-32 textarea textarea-bordered textarea-md"
                defaultValue={profile.bio}
                {...register("bio")}
              />
            </div>
            {/* Username */}
            <div className="flex flex-col">
              <div className="flex mb-1 h-full items-center pr-4">Username</div>
              <input
                type="text"
                placeholder="Type here"
                className="input input-bordered input-md w-96"
                defaultValue={profile.username}
                {...register("username", {
                  minLength: 4,
                  maxLength: 20,
                  required: true,
                })}
              />
              {errors.username?.type === "required" && (
                <p className="flex mb-1 h-full  pl-4 bg-purple-700">
                  Username is required to update profile
                </p>
              )}
              {errors.username?.type === "min" && (
                <p className="flex mb-1 h-full justify-center items-center pl-4">
                  Username must be 4 long
                </p>
              )}
              {errors.username?.type === "max" && (
                <p className="flex mb-1 h-full justify-center items-center pl-4">
                  Username need to be less than 20 characters long
                </p>
              )}
            </div>
            {/* Full Name */}
            <div className="flex flex-row">
              <div className="flex flex-col mb-5 w-96">
                <div className="flex h-full items-center pr-4">Full Name</div>
                <input
                  type="text"
                  placeholder="Type here"
                  className="input input-bordered input-md"
                  defaultValue={profile.full_name}
                  {...register("full_name", {
                    minLength: 2,
                    maxLength: 60,
                  })}
                />
              </div>
              {errors.full_name?.type === "min" && (
                <p className="mb-1 h-full justify-center items-center pl-4">
                  Name must be 2 long
                </p>
              )}
              {errors.full_name?.type === "max" && (
                <p className="flex mb-1 h-full justify-center items-center pl-4">
                  Name need to be less than 60 characters long
                </p>
              )}
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
