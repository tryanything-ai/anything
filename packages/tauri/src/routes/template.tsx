import { TemplateView } from "@anything/ui";
import { BigFlow, Profile } from "@anything/utils";
import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";

import { Avatar } from "../components/avatar";
import { useMarketplaceContext } from "../context/MarketplaceProvider";

const Template = () => {
  const { slug } = useParams<{
    slug: string;
  }>();

  const { fetchTemplateBySlug, fetchProfile } = useMarketplaceContext();
  const [template, setTemplate] = useState<BigFlow>();
  const [profile, setProfile] = useState<Profile>();

  const fetchTemplate = async () => {
    if (!slug) {
      console.log("Author username or template name not found.");
      return;
    }
    let templateResponse = await fetchTemplateBySlug(slug);
    if (!templateResponse) return;

    let template: BigFlow = templateResponse[0];

    console.log("template in TemplatePage", JSON.stringify(template, null, 3));
    setTemplate(template);

    let profile: Profile | undefined = template?.profiles?.username
      ? await fetchProfile(template.profiles.username)
      : undefined;

    // let template = await fetchTemplate(author_username, template_name);
    console.log(template);
    setProfile(profile);
  };

  useEffect(() => {
    if (!slug) {
      console.log("Slug not found.");
      return;
    }
    fetchTemplate();
  }, [slug]);

  return (
    <div className="hide-scrollbar mx-4 my-6 flex h-full min-h-screen max-w-4xl flex-col overflow-scroll md:mx-auto md:py-16">
      {template ? (
        <TemplateView
          template={template}
          profile={profile}
          Link={Link}
          Avatar={Avatar}
        />
      ) : (
        "loading?"
      )}
    </div>
  );
};

export default Template;
