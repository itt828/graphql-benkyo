import { ApolloClient, createHttpLink, InMemoryCache } from '@apollo/client/core'

export const httpLink = createHttpLink({
  uri: 'http://localhost:3000/'
})
export const cache = new InMemoryCache()

export const apolloClient = new ApolloClient({
  link: httpLink,
  cache
})
