<template>
  <div>
    <h2>Home</h2>
    <p>Here's some asyncData: {{ id }}</p>
    <router-link to="/other">Go to /other</router-link>
  </div>
</template>

<script>
export default {
  data () {
    return {
      id: $ssrContext.data,
    }
  }
}
</script>
