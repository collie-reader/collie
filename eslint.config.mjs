import js from "@eslint/js";
import tseslint from "typescript-eslint";
import solid from "eslint-plugin-solid/configs/typescript";
import globals from "globals";

export default tseslint.config(
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ["**/*.{ts,tsx}"],
    ...solid,
    languageOptions: {
      globals: {
        ...globals.browser,
      },
      parserOptions: {
        project: "tsconfig.json",
      },
    },
    rules: {
      "no-shadow": "off",
      "@typescript-eslint/no-shadow": "error",
      "no-unused-vars": "off",
      "@typescript-eslint/no-unused-vars": "error",
    },
  },
  {
    ignores: ["dist/**", "node_modules/**", "src-tauri/**"],
  }
);
