import { MetadataRoute } from 'next'

const base_url = process.env.NEXT_PUBLIC_HOSTED_URL;

export default function robots(): MetadataRoute.Robots {
  return {
    rules: {
      userAgent: '*',
    },
    sitemap: `${base_url}/sitemap.xml`,
  }
}