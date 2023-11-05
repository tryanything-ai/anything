const { resolve } = require("node:path");

const project = resolve(process.cwd(), "tsconfig.json");

/*
 * This is a custom ESLint configuration for use with
 * Next.js apps.
 *
 * This config extends the Vercel Engineering Style Guide.
 * For more information, see https://github.com/vercel/style-guide
 *
 */

module.exports = {
  extends: [
    "@vercel/style-guide/eslint/node",
    "@vercel/style-guide/eslint/typescript",
    "@vercel/style-guide/eslint/browser",
    "@vercel/style-guide/eslint/react",
    "@vercel/style-guide/eslint/next",
    "eslint-config-turbo",
  ].map(require.resolve),
  parserOptions: {
    project,
  },
  globals: {
    React: true,
    JSX: true,
  },
  settings: {
    "import/resolver": {
      typescript: {
        project,
      },
      node: {
        extensions: [".mjs", ".js", ".jsx", ".ts", ".tsx"],
      },
    },
  },
  ignorePatterns: ["node_modules/", "dist/"],
  // add rules configurations here
  rules: {
    //TODO: turn all back on probably
    "import/no-default-export": "off",
    "@typescript-eslint/no-explicit-any": "off",
    "@typescript-eslint/no-unused-vars": "off",
    "import/no-unresolved": "off",
    "no-unused-vars": "off",
    "@typescript-eslint/consistent-type-definitions": "off",
    "@typescript-eslint/consistent-indexed-object-style": "off",
    "unicorn/filename-case": "off",
    "import/no-extraneous-dependencies": "off",
    "@typescript-eslint/no-unsafe-member-access": "off",
    "@typescript-eslint/no-unsafe-argument": "off",
    "turbo/no-undeclared-env-vars": "off",
    "no-console": "off",
    "@typescript-eslint/no-unsafe-call": "off",
    "@typescript-eslint/no-unsafe-return": "off",
    camelcase: "off",
    "@typescript-eslint/no-throw-literal": "off",
    "@typescript-eslint/no-unnecessary-condition": "off",
    "@typescript-eslint/no-confusing-void-expression": "off",
    "@typescript-eslint/no-confusing-void-expression": "off",
    "@typescript-eslint/no-floating-promises": "off",
    "@typescript-eslint/ban-types": "off", 
    "@typescript-eslint/consistent-type-imports": "off",
    "@typescript-eslint/require-await": "off", 
    "@typescript-eslint/prefer-optional-chain": "off",
    "@typescript-eslint/no-misused-promises": "off", 
    "@typescript-eslint/no-unsafe-call": "off", 
    "no-alert": "off", 
    "@typescript-eslint/no-unsafe-assignment": "off", 
    "no-undef": "off", 
  },
};
