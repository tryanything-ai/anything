import type { Config } from "tailwindcss";
import sharedConfig from "tailwind-config/tailwind.config.ts";

const config: Pick<Config, "presets"> = {
  presets: [sharedConfig as Config],
};

export default config;