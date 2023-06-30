<script setup lang="ts">
import { RouterView } from 'vue-router'
import { provide } from 'vue'
import { DefaultApolloClient } from '@vue/apollo-composable'
import { apolloClient } from '@/api/graphql'

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
