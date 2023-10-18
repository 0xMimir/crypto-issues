<script setup lang="ts">
import NavBar from "@/components/NavBar.vue";
</script>
<template>
  <div>
    <NavBar />
    <div class="wrapper">
      <RouterView />
    </div>
  </div>
</template>
<style scoped>
.wrapper {
  width: 80%;
  margin: auto;
}
</style>
