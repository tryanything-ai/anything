module.exports = {
  images: {
    loader: "custom",
    loaderFile: "./src/lib/supabaseImageLoader.ts",
  },
  transpilePackages: ["@repo/ui"],
  //https://turbo.build/repo/docs/handbook/sharing-code/internal-packages
};
