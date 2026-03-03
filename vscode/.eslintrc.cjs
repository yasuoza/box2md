module.exports = {
  root: true,
  env: {
    node: true,
    mocha: true,
    es2022: true
  },
  parser: '@typescript-eslint/parser',
  parserOptions: {
    project: './tsconfig.json',
    sourceType: 'module'
  },
  plugins: ['@typescript-eslint'],
  extends: ['eslint:recommended', 'plugin:@typescript-eslint/recommended'],
  ignorePatterns: ['out/**'],
  rules: {
    '@typescript-eslint/no-floating-promises': 'error'
  }
};
