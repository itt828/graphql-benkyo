<template>
  {{ blog }}
</template>
<script setup lang="ts">
import { graphql } from '@/api/graphql/generated'
import { useQuery } from '@vue/apollo-composable'
import { computed } from 'vue'

const { result } = useQuery(
  graphql(`
    query blog($id: ID!) {
      blog(id: $id) {
        id
        title
        content
        tags {
          id
        }
      }
    }
  `),
  { id: '9778228a-12d2-4a22-93ff-e571cf7f7e18' }
)
const blog = computed(() => result.value)
</script>
