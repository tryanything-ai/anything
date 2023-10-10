// Profile page
// `pages` directory
// import ProfileLayout from '@/components/post-layout'
// import { GetStaticPathsContext, GetStaticPropsContext, NextPageContext } from "next"
import { fetchProfiles, fetchProfile } from "@/lib/fetchSupabase";
import { Database } from "@/types/supabase.types";
import { GetStaticProps, GetStaticPaths, GetServerSideProps } from "next";
import { notFound } from "next/navigation";

type Profile = Database['public']['Tables']['profiles']['Row']

// export const getStaticPaths: GetStaticPaths = async () => {
//   //TODO: fetch profiles from db
//   return {
//     paths: [
//       { params: { username: "carl" } },
//       { params: { username: "jimbo" } },
//     ],
//     fallback: true, //create it if it didnt exist before
//   };
// };

// export const getStaticProps: GetStaticProps = async ({ params }) => {
//   //TODO:  get all profile data
//   // const res = await fetch(`https://.../posts/${params?.id}`)
//   // const post = await res.json()

// };

export const generateStaticParams = async () => {
  // const posts = await fetch('https://.../posts').then((res) => res.json())

  // return posts.map((post) => ({
  //   slug: post.slug,
  // }))
  // return { props: { post: { ...params, content: "derp" } } };
  // return [{ username: "carl" }, { username: "jim" }];

  let profiles = await fetchProfiles(); 
  // console.log("profiles", profiles);
  return profiles; 
};

// async function getProfile(params: any) {
//   console.log("params", params);
  
//   // const res = await fetch(`https://.../posts/${params.id}`)
//   // const post = await res.json()
//   // console.log("username", username);
//   // if (username != "carl" && username != "jim") {
//   //   return undefined;
//   // } else {
//     return { ...params, content: "derp" };
//   // }
// }

export default async function Profile({ params }: any) {
  const profile = await fetchProfile(params.username);

  if (!profile) {
    notFound();
  }
  
  return <div>{JSON.stringify(profile, null, 3)}</div>;
  // return <PostLayout post={post} />
}
