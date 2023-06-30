import type { CodegenConfig } from '@graphql-codegen/cli'

const config: CodegenConfig = {
  overwrite: true,
  schema: '../schema.graphql',
  documents: 'src/**/*.vue',
  generates: {
    'src/api/graphql/generated/': {
      preset: 'client'
      // plugins: ['typescript', 'typescript-operations', 'typescript-vue-apollo']
    }
  }
}

export default config
