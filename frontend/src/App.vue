<script setup lang="ts">
import { RouterView } from 'vue-router'
import { provide } from 'vue'
import { DefaultApolloClient } from '@vue/apollo-composable'
import { apolloClient } from '@/api/graphql'
import router from './router'

provide(DefaultApolloClient, apolloClient)
const moveTop = () => {
  router.push({ path: '/' })
}
const moveNew = () => {
  router.push({ path: '/posts/new' })
}
</script>

<template>
  <header :class="$style.header">
    <div @click="moveTop" style="cursor: pointer">ブログ</div>
    <button @click="moveNew">+ 新しく作成</button>
  </header>
  <main>
    <RouterView />
  </main>
</template>

<style module lang="scss">
html {
  font-family: Arial, Helvetica, sans-serif;
}
.header {
  font-size: 1.5rem;
  display: flex;
}
</style>
