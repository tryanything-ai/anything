module.exports = {
  reactStrictMode: true,
  images: {
    loader: "custom",
    loaderFile: "./src/lib/supabaseImageLoader.ts",
  },
  // transpilePackages: ["utils", "ui"], //recommended here
  //https://turbo.build/repo/docs/handbook/sharing-code/internal-packages
};
