import api from "@repo/anything-api";
import { MetadataRoute } from "next";

const base_url = process.env.NEXT_PUBLIC_HOSTED_URL;

async function getBlogSitemap() {
  const key = process.env.SEOBOT_API_KEY;
  if (!key) throw Error('SEOBOT_API_KEY enviroment variable must be set. You can use the DEMO key a8c58738-7b98-4597-b20a-0bb1c2fe5772 for testing - please set it in the root .env.local file.');

  try {
    const res = await fetch(`https://app.seobotai.com/api/sitemap?key=${key}`, { cache: 'no-store' });
    const result = await res.json();
    return result?.data || { articles: [], categories: [], tags: [] };
  } catch {
    return { articles: [], categories: [], tags: [] };
  }
}

export default async function sitemap(): Promise<MetadataRoute.Sitemap> {
  const routes: any = [];

  //TODO: works for more than 1000? 100? idk
  // const templateResult = await api.marketplace.getWorkflowTemplatesForMarketplace();

  // if (templateResult) {
  //   templateResult.forEach((template: any) =>
  //     routes.push({
  //       url: `${base_url}/templates/${template.slug}`,
  //       lastModified: new Date(template.created_at),
  //       changeFrequency: "monthly",
  //       priority: 0.3,
  //     })
  //   );
  // }

  // const profileResult = await fetchProfiles();

  // if (profileResult) {
  //   profileResult.forEach((profile) =>
  //     routes.push({
  //       url: `${base_url}/${profile.username}`,
  //       lastModified: profile.updated_at ? new Date(profile.updated_at) : null,
  //       changeFrequency: "monthly",
  //       priority: 0.1,
  //     })
  //   );
  // }

  //home
  routes.push({
    url: `${base_url}`,
    lastModified: new Date(),
    changeFrequency: "weekly",
    priority: 1,
  });

  //templates
  // routes.push({
  //   url: `${base_url}/templates/workflows`,
  //   lastModified: new Date(),
  //   changeFrequency: "weekly",
  //   priority: 0.8,
  // });

  //integrations aka action templates
  //  routes.push({
  //   url: `${base_url}/templates/actions`,
  //   lastModified: new Date(),
  //   changeFrequency: "weekly",
  //   priority: 0.8,
  // });

  // Add blog routes
  const blogSitemap = await getBlogSitemap();
  
  // Add main blog page
  routes.push({
    url: `${base_url}/blog`,
    lastModified: new Date(),
    changeFrequency: "daily",
    priority: 0.9,
  });

  // Add blog articles
  blogSitemap.articles.forEach((article: { slug: string; lastmod: string }) => {
    routes.push({
      url: `${base_url}/blog/${article.slug}`,
      lastModified: new Date(article.lastmod),
      changeFrequency: "weekly",
      priority: 0.7,
    });
  });

  // Add blog categories
  blogSitemap.categories.forEach((category: { slug: string; lastmod: string }) => {
    routes.push({
      url: `${base_url}/blog/category/${category.slug}`,
      lastModified: new Date(category.lastmod),
      changeFrequency: "weekly",
      priority: 0.6,
    });
  });

  // Add blog tags
  blogSitemap.tags.forEach((tag: { slug: string; lastmod: string }) => {
    routes.push({
      url: `${base_url}/blog/tag/${tag.slug}`,
      lastModified: new Date(tag.lastmod),
      changeFrequency: "weekly",
      priority: 0.5,
    });
  });

  return [...routes];
}