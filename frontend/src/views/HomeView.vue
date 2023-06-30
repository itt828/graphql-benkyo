<script setup lang="ts">
import { apolloClient } from '@/api/graphql'
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
import { computed } from 'vue'

// const query = provideApolloClient(apolloClient)(() =>
//   useQuery(gql`
//     query {
//       user {
//         id
//         name
//       }
//     }
//   `)
// )
// const result = computed(() => query.result.value)
</script>

<template>
  d
  <!-- <main>result: {{ result }}</main> -->
</template>
