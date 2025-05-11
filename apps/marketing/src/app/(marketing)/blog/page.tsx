import ArticleCard from "@/components/blog/ArticleCard";
import Pagination from "@/components/blog/Pagination";
import { type Metadata } from "next";
import { BlogClient } from "seobot";
import { siteConfig } from "@/config/site";
export async function generateMetadata(): Promise<Metadata> {
  const title = "Anything AI Blog";
  const description =
    "Get the inside scoop on Anything AI - the AI-powered Automation solution for your business.";
  return {
    title,
    description,
    metadataBase: new URL(siteConfig.url),
    alternates: {
      canonical: `${siteConfig.url}/blog`,
    },
    openGraph: {
      type: "website",
      title,
      description,
      // images: [],
      url: `${siteConfig.url}/blog`,
    },
    twitter: {
      title,
      description,
      // card: 'summary_large_image',
      // images: [],
    },
  };
}

async function getPosts(page: number) {
  const key = process.env.NEXT_PUBLIC_SEOBOT_API_KEY;
  if (!key)
    throw Error(
      "NEXT_PUBLIC_SEOBOT_API_KEY enviroment variable must be set. You can use the DEMO key a8c58738-7b98-4597-b20a-0bb1c2fe5772 for testing - please set it in the root .env.local file",
    );

  const client = new BlogClient(key);
  return client.getArticles(page, 10);
}

export const fetchCache = "force-no-store";

export default async function Blog({
  searchParams: { page },
}: {
  searchParams: { page: number };
}) {
  const pageNumber = Math.max((page || 0) - 1, 0);
  const { total, articles } = await getPosts(pageNumber);
  const posts = articles || [];
  const lastPage = Math.ceil(total / 10);

  return (
    <section className="max-w-3xl my-8 lg:mt-10 mx-auto px-4 md:px-8 dark:text-white tracking-normal">
      <h1 className="text-4xl my-4 font-black">Anything AI Blog</h1>
      <ul>
        {posts.map((article: any) => (
          <ArticleCard key={article.id} article={article} />
        ))}
      </ul>
      {lastPage > 1 && (
        <Pagination slug="/blog" pageNumber={pageNumber} lastPage={lastPage} />
      )}
    </section>
  );
}
