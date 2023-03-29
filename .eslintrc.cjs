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
  // only config tsconfig in overrides is the key to lint both svelte and ts
  overrides: [
    {
      files: ['*.svelte'],
      processor: 'svelte3/svelte3',
      rules: {
        // https://stackoverflow.com/questions/63509978/global-module-definition-for-typescript
        // https://typescript-eslint.io/linting/troubleshooting/#i-get-errors-from-the-no-undef-rule-about-global-variables-not-being-defined-even-though-there-are-no-typescript-errors
        'no-undef': 'off',
        // https://github.com/sveltejs/eslint-plugin-svelte3/issues/73
        'import/first': 'off',

        // https://github.com/typescript-eslint/typescript-eslint/issues/2621
        'no-unused-vars': 'off',
        '@typescript-eslint/no-unused-vars': 'error'
      }
    },
    {
      files: [
        '*.ts'
      ],
      extends: [
        'standard-with-typescript'
      ],
      parserOptions: {
        project: 'tsconfig.json'
      },
      rules: {
        'no-undef': 'off'
      }
    }
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module'
  },
  rules: {
    // weird bug with svelte? when max = 1
    'no-multiple-empty-lines': ['error', { max: 2, maxEOF: 1 }]
  },
  settings: {
    'svelte3/typescript': require('typescript')
  }

}

