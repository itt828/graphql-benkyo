<script setup lang="ts">
import { computed } from 'vue'
import { graphql } from '@/api/graphql/generated'
import { useMutation, useQuery } from '@vue/apollo-composable'
import router from '@/router'

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

const { mutate: login } = useMutation(
  graphql(`
    mutation Login {
      login
    }
  `)
)

const onClick = async () => {
  const result = await login()
  window.location.assign(result?.data?.login!)
}
</script>

<template>
  <button @click="onClick">ログイン</button>
  <div v-for="blog in blogs" :key="blog.id">
    <RouterLink :to="`/posts/${blog.id}`">
      {{ blog.title }}
    </RouterLink>
  </div>
</template>
