<script setup lang="ts">
import { computed } from 'vue'
import { graphql } from '@/api/graphql/generated'
import { useQuery } from '@vue/apollo-composable'

const { result } = useQuery(
  graphql(`
    query blogs {
      blogs {
        id
        title
      }
    }
  `)
)
const blogs = computed(() => result.value?.blogs)
</script>

<template>
  <div v-for="blog in blogs" :key="blog.id">
    <RouterLink :to="`/posts/${blog.id}`">
      {{ blog.title }}
    </RouterLink>
  </div>
</template>
