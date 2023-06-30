import type { CodegenConfig } from '@graphql-codegen/cli'

const config: CodegenConfig = {
  overwrite: true,
  schema: 'src/api/graphql/schema.graphql',
  documents: 'src/**/*.vue',
  generates: {
    'src/api/graphql/generated/': {
      preset: 'client',
      plugins: ['typescript', 'typescript-operations', 'typescript-vue-apollo'],
      config: {
        // withCompositionFunctions: true
        useTypeImports: true
      }
    }
  }
}

export default config
