module.exports = {
  extends: ["custom/react"],
  rules: {
    "@typescript-eslint/no-explicit-any": "off",
    "react/function-component-definition": [
      "error",
      {
        namedComponents: "arrow-function",
      },
    ],
    "@typescript-eslint/explicit-function-return-type": "off"
  },
};
