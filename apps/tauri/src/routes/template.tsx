import { TemplateView } from "ui";
import { BigFlow, Profile } from "utils";
import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";

import { Avatar } from "../components/avatar";
import { useMarketplaceContext } from "../context/MarketplaceProvider";
import PageLayout from "../pageLayout";

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

    let template: any = templateResponse[0];

    console.log("template in TemplatePage", JSON.stringify(template, null, 3));
    setTemplate(template);

    let profile: Profile | undefined = template?.profiles?.username
      ? await fetchProfile(template.profiles.username)
      : undefined;

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

  // let flow: Flow = flowJsonFromBigFlow(template);
  let t = template as any;

  const Action = () => {
    return (
      <>
        {profile ? (
          <div
            className="btn btn-primary"
            data-ph-capture-attribute-flow-template-name={t.flow_template_name}
            data-ph-capture-attribute-flow-template-slug={t.slug}
            data-ph-capture-attribute-flow-template-id={t.flow_template_id}
            data-ph-capture-attribute-flow-template-profile-username={
              profile.username
            }
            data-ph-capture-attribute-flow-template-profile-id={profile.id}
          >
            Use Template
          </div>
        ) : null}
      </>
    );
  };

  return (
    <PageLayout>
      <div className="w-2/3 max-w-4xl mx-auto">
        {template ? (
          <TemplateView
            ActionComponent={Action}
            template={template}
            profile={profile}
            Link={Link}
            Avatar={Avatar}
          />
        ) : (
          "loading?"
        )}
      </div>
    </PageLayout>
  );
};

export default Template;
