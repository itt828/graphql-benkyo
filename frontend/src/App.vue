<script setup lang="ts">
import { RouterView } from 'vue-router'
import { provide } from 'vue'
import { DefaultApolloClient } from '@vue/apollo-composable'
import { apolloClient } from '@/api/graphql'

provide(DefaultApolloClient, apolloClient)
</script>

<template>
  <header :class="$style.header">
    <div>ブログ</div>
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
}
</style>
