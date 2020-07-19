<template>
  <div>
    <h1>Default layout</h1>
    <router-view></router-view>
  </div>
</template>

<script>
export default {
  mounted () {
    window.x = this
  }
}
</script>