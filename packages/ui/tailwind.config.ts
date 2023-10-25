// tailwind config is required for editor support
import type { Config } from "tailwindcss";
import sharedConfig from "tailwind-config/tailwind.config.ts";

const config: Pick<Config, "presets" | "prefix"> = {
  // prefix: "derp-",
  presets: [sharedConfig],
};

export default config;
