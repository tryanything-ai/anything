import { MetadataRoute } from 'next'
let base_url = "https://" + process.env.NEXT_PUBLIC_VERCEL_URL; 
 
export default function robots(): MetadataRoute.Robots {
  return {
    rules: {
      userAgent: '*',
      allow: '/',
      disallow: '/private/',
    },
    sitemap: base_url + '/sitemap.xml',
  }
}