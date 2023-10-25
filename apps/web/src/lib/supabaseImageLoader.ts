// Docs: https://supabase.com/docs/guides/storage/image-transformations#nextjs-loader
// import { env } from "@/env.mjs";

export function isRelativeUrl(url: string): boolean {
  return url.startsWith('/');
}

export default function supabaseLoader({ src, width, quality }: any) {
  if(isRelativeUrl(src)) return src;
  let url = new URL(src);
  if (url.protocol + "//" + url.hostname !== process.env.NEXT_PUBLIC_SUPABASE_URL) {
    console.log(url.hostname);
    console.log(url.protocol);
    console.log(process.env.NEXT_PUBLIC_SUPABASE_URL);
    throw new Error("Hostname does not match .env variable");
  }

  url.searchParams.set("width", width.toString());
  url.searchParams.set("quality", (quality || 75).toString());
  return url.href;
}
