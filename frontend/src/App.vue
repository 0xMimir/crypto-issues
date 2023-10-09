<script setup lang="ts"></script>

<template>
  <header>
    <RouterLink to="/" class="link">Crypto Issues</RouterLink>
  </header>
  <br />
  <div class="wrapper">
    <RouterView />
  </div>
</template>
<style scoped>
header {
  font-size: x-large;
  padding: 0.5rem;
  font-size: xx-large;
}
.link,
.link :visited {
  color: inherit;
  text-decoration: none;
}
.wrapper {
  width: 80%;
  margin: auto;
}
</style>
