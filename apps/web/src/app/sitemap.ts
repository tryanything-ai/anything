import { fetchTemplates, fetchProfiles } from "utils";
import { MetadataRoute } from "next";

let base_url = "https://" + process.env.NEXT_PUBLIC_VERCEL_URL;

export default async function sitemap(): Promise<MetadataRoute.Sitemap> {
  let routes: any = [];

  //TODO: works for more than 1000? 100? idk 
  const templateResult = await fetchTemplates();

  if (templateResult) {
    templateResult.forEach((template) =>
      routes.push({
        url: `${base_url}/${template.slug}`,
        lastModified: template.created_at,
        changeFrequency: "monthly",
      })
    );
  }

  const profileResult = await fetchProfiles();

  if (profileResult) {
    profileResult.forEach((profile) =>
      routes.push({
        url: `${base_url}/${profile.username}`,
        lastModified: profile.updated_at,
        changeFrequency: "yearly",
      })
    );
  }

  //home
  routes.push({
    url: `${base_url}`,
    lastModified: new Date(),
    changeFrequency: "monthly",
  });

  //templates
  routes.push({
    url: `${base_url}/templates`,
    lastModified: new Date(),
    changeFrequency: "daily",
  });

  return [...routes];
}