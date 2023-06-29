<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import HelloWorld from './components/HelloWorld.vue'
import { provide } from 'vue'
import { DefaultApolloClient } from '@vue/apollo-composable'
import { apolloClient } from '@/api/graphql'
import 'normalize.css'

provide(DefaultApolloClient, apolloClient)
</script>

<template>
  <header>
    <div>header</div>
  </header>
  <main>
    <RouterView />
  </main>
</template>

<style lang="scss">
html {
  font-family: Arial, Helvetica, sans-serif;
}
</style>
