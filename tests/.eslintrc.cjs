module.exports = {
  env: {
    browser: true,
    es2021: true,
  },
  extends: "standard-with-typescript",
  overrides: [
    {
      env: {
        node: true,
      },
      files: [".eslintrc.{js,cjs}"],
      parserOptions: {
        sourceType: "script",
      },
    },
  ],
  parserOptions: {
    ecmaVersion: "latest",
    sourceType: "module",
    project: true
  },
  ignorePatterns: ["react-app-env.d.ts", "reportWebVitals.ts"],
  rules: {
    "@typescript-eslint/explicit-function-return-type": 0,
    "@typescript-eslint/space-before-function-paren": 0,
    "@typescript-eslint/comma-dangle": 0,
    "@typescript-eslint/prefer-nullish-coalescing" : 0,
    "@typescript-eslint/strict-boolean-expressions" : 0,
    "@typescript-eslint/restrict-plus-operands" : 0,
    "@typescript-eslint/no-base-to-string" : 0,
    "@typescript-eslint/restrict-template-expressions" : 0
  },
};
