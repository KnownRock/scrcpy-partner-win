module.exports = {
  parser: '@typescript-eslint/parser',
  env: {
    browser: true,
    es2021: true
  },
  extends: 'standard-with-typescript',
  plugins: [
    '@typescript-eslint',
    'svelte3'
  ],
  overrides: [
    {
      files: ['*.svelte'],
      processor: 'svelte3/svelte3'
    }
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module'
    // project: './tsconfig.node.json'
  },
  rules: {
    // weird bug with svelte? when max = 1
    'no-multiple-empty-lines': ['error', { max: 2, maxEOF: 1 }]
  },
  settings: {
    'svelte3/typescript': require('typescript')
  }
}
