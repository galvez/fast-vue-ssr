<template>
  <div id="app">
    <component :is="`${layout}-layout`"></component>
  </div>
</template>

<script>
import defaultLayout from './layouts/default.vue'
export default {
  name: "App",
  data: () => ({
    layout: 'default',
  }),
  components: {
    defaultLayout,
  },
}
</script>
