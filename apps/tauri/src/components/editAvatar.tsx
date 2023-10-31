import { useEffect, useState } from "react";

import { useMarketplaceContext } from "../context/MarketplaceProvider";
import { useAuthenticationContext } from "../context/AuthenticaionProvider";

export default function EditAvatar({ profile_id, avatar_url, size }) {
  const [avatarUrl, setAvatarUrl] = useState(avatar_url);
  const [uploading, setUploading] = useState(false);
  const { uploadAvatar } = useMarketplaceContext();
  const { fetchProfile } = useAuthenticationContext();

  async function _uploadAvatar(event) {
    try {
      setUploading(true);

      if (!event.target.files || event.target.files.length === 0) {
        throw new Error("You must select an image to upload.");
      }

      const file = event.target.files[0];
      const fileExt = file.name.split(".").pop();
      const fileName = `${Math.random()}.${fileExt}`;
      const filePath = `${fileName}`;

      const result = await uploadAvatar(profile_id, filePath, file);

      if (!result) {
        throw new Error("No result from Upload Avatar");
      }

      setAvatarUrl(result.avatar_url);
      //refresh profile in user authentication system
      fetchProfile();
    } catch (error) {
      console.log("Error uploading avatar: ", error);
    } finally {
      setUploading(false);
    }
  }

  return (
    <div>
      {avatarUrl ? (
        <div className="avatar ml-10 mb-5">
          <div className="w-56 h-56 rounded-full">
            <img src={avatarUrl} alt={""} />
          </div>
        </div>
      ) : (
        <div className="avatar  ml-10 mb-5">
          <div className="w-56 h-56 rounded-full">
            <img
              src={
                "https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/public/mocks/botttsNeutral-1698715092376.png"
              }
              alt={""}
            />
          </div>
        </div>
      )}
      <div style={{ width: size }}>
        <label htmlFor="single" className="btn btn-primary w-56 ml-10">
          {uploading ? "Uploading ..." : "Upload"}
        </label>

        <input
          style={{
            visibility: "hidden",
            position: "absolute",
          }}
          type="file"
          id="single"
          accept="image/*"
          onChange={_uploadAvatar}
          disabled={uploading}
        />
      </div>
    </div>
  );
}
