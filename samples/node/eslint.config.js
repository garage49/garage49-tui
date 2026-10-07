import tseslint from 'typescript-eslint';

/** The complexity gate: cyclomatic complexity, function length and parameter count, parsed as TypeScript/TSX. */
export default tseslint.config(
  {ignores: ['node_modules/**', 'dist/**']},
  {
    files: ['src/**/*.ts', 'src/**/*.tsx', 'test/**/*.ts'],
    languageOptions: {parser: tseslint.parser, parserOptions: {ecmaFeatures: {jsx: true}}},
    rules: {
      complexity: ['error', 10],
      'max-lines-per-function': ['error', {max: 50, skipBlankLines: true, skipComments: true}],
      'max-params': ['error', 5],
    },
  },
);
