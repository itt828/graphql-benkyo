<template>
  <div :class="$style.container">
    <button @click="onSubmit">送信</button>
    <input placeholder="タイトル" v-model="title" />
    <textarea placeholder="本文" style="height: 880px" v-model="content"></textarea>
  </div>
</template>
<script setup lang="ts">
import { graphql } from '@/api/graphql/generated'
import router from '@/router'
import { useMutation } from '@vue/apollo-composable'
import { ref } from 'vue'

const title = ref('')
const content = ref('')

const { mutate: postBlog } = useMutation(
  graphql(`
    mutation AddBlogMutation($title: String!, $content: String!) {
      addBlog(title: $title, content: $content) {
        id
      }
    }
  `)
)

const onSubmit = async () => {
  const result = await postBlog({ title: title.value, content: content.value })
  const redirectId = result?.data?.addBlog.id
  router.push({ path: `/posts/${redirectId}` })
}
</script>

<style module lang="scss">
.container {
  font-size: 1.5rem;
  padding: 2rem 1rem;
  max-width: 960px;
  display: flex;
  flex-direction: column;
}
</style>
