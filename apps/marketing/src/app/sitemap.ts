import { fetchTemplates, fetchProfiles } from "utils";
import { MetadataRoute } from "next";

const base_url = process.env.NEXT_PUBLIC_HOSTED_URL;

export default async function sitemap(): Promise<MetadataRoute.Sitemap> {
  const routes: any = [];

  //TODO: works for more than 1000? 100? idk
  const templateResult = await fetchTemplates();

  if (templateResult) {
    templateResult.forEach((template) =>
      routes.push({
        url: `${base_url}/templates/${template.slug}`,
        lastModified: new Date(template.created_at),
        changeFrequency: "monthly",
        priority: 0.3,
      })
    );
  }

  const profileResult = await fetchProfiles();

  if (profileResult) {
    profileResult.forEach((profile) =>
      routes.push({
        url: `${base_url}/${profile.username}`,
        lastModified: profile.updated_at ? new Date(profile.updated_at) : null,
        changeFrequency: "monthly",
        priority: 0.1,
      })
    );
  }

  //home
  routes.push({
    url: `${base_url}`,
    lastModified: new Date(),
    changeFrequency: "daily",
    priority: 1,
  });

  //templates
  routes.push({
    url: `${base_url}/templates`,
    lastModified: new Date(),
    changeFrequency: "daily",
    priority: 0.8,
  });

  return [...routes];
}