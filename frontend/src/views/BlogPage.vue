<template>
  <div :class="$style.container">
    <h1>
      {{ blog?.title }}
    </h1>
    <div>
      {{ blog?.content }}
    </div>
  </div>
</template>
<script setup lang="ts">
import { graphql } from '@/api/graphql/generated'
import { useQuery } from '@vue/apollo-composable'
import { computed } from 'vue'
import { useRoute } from 'vue-router'

// const props = defineProps<{
//   blog_id: string
// }>()

const route = useRoute()

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
  { id: route.params.id as string }
)
const blog = computed(() => result.value?.blog)
</script>

<style module lang="scss">
.container {
  font-size: 1.5rem;
  padding: 2rem 1rem;
  max-width: 960px;
}
</style>
