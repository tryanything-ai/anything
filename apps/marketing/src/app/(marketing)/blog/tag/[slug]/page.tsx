import ArticleCard from "@/components/blog/ArticleCard";
import Pagination from "@/components/blog/Pagination";
import { type Metadata } from "next";
import Link from "next/link";
import { BlogClient } from "seobot";
import { siteConfig } from "@/config/site";
async function getPosts(slug: string, page: number) {
  const key = process.env.NEXT_PUBLIC_SEOBOT_API_KEY;
  if (!key)
    throw Error(
      "NEXT_PUBLIC_SEOBOT_API_KEY enviroment variable must be set. You can use the DEMO key a8c58738-7b98-4597-b20a-0bb1c2fe5772 for testing - please set it in the root .env.local file.",
    );

  const client = new BlogClient(key);
  return client.getTagArticles(slug, page, 10);
}

function deslugify(str: string) {
  return str.replace(/-/g, " ").replace(/\b\w/g, (char) => char.toUpperCase());
}

export const fetchCache = "force-no-store";

export async function generateMetadata({
  params: { slug },
}: {
  params: { slug: string };
}): Promise<Metadata> {
  const title = `${deslugify(slug)} - Anything AI Blog`;
  return {
    title,
    metadataBase: new URL(siteConfig.url),
    alternates: {
      canonical: `${siteConfig.url}/blog/tag/${slug}`,
    },
    openGraph: {
      type: "article",
      title,
      // description: '',
      // images: [],  
      url: `${siteConfig.url}/blog/tag/${slug}`,
    },
    twitter: {  
      title,
      // description: '',
      // card: 'summary_large_image',
      // images: [],
    },
  };
}

export default async function Tag({
  params: { slug },
  searchParams: { page },
}: {
  params: { slug: string };
  searchParams: { page: number };
}) {
  const pageNumber = Math.max((page || 0) - 1, 0);
  const { total, articles } = await getPosts(slug, pageNumber);
  const posts = articles || [];
  const lastPage = Math.ceil(total / 10);

  return (
    <section className="max-w-3xl my-8 lg:mt-10 mx-auto px-4 md:px-8 dark:text-white">
      <div className="flex flex-wrap items-center gap-2 mb-1 w-full text-gray-600 dark:text-slate-400 text-sm mb-4">
        <Link
          href="/"
          className="text-pink-600 dark:text-pink-400 hover:text-pink-700 dark:hover:text-pink-300 transition-colors"
        >
          Home
        </Link>
        <svg
          width="12"
          height="12"
          viewBox="0 0 1024 1024"
          xmlns="http://www.w3.org/2000/svg"
          className="text-gray-400 dark:text-slate-600"
        >
          <path
            fill="currentColor"
            d="M338.752 104.704a64 64 0 0 0 0 90.496l316.8 316.8l-316.8 316.8a64 64 0 0 0 90.496 90.496l362.048-362.048a64 64 0 0 0 0-90.496L429.248 104.704a64 64 0 0 0-90.496 0z"
          />
        </svg>
        <Link
          href="/blog/"
          className="text-pink-600 dark:text-pink-400 hover:text-pink-700 dark:hover:text-pink-300 transition-colors"
        >
          Blog
        </Link>
      </div>
      <h1 className="text-4xl my-4 font-black text-gray-900 dark:text-white">
        Tag: {deslugify(slug)}
      </h1>
      <ul>
        {posts.map((article: any) => (
          <ArticleCard key={article.id} article={article} />
        ))}
      </ul>
      {lastPage > 1 && (
        <Pagination
          slug={`/blog/tag/${slug}`}
          pageNumber={pageNumber}
          lastPage={lastPage}
        />
      )}
    </section>
  );
}
