<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useRouter, useRoute } from "vue-router";

const router = useRouter();
const route = useRoute();

const active = ref(0);
const items = ref([
  {
    label: "Home",
    route: "/",
  },
  {
    label: "Statistics",
    route: "/statistics",
  },
  {
    label: 'Repositories',
    route: "/repository"
  },
  {
    label: "Projects",
    route: "/projects"
  }
]);

onMounted(() => {
  active.value = items.value.findIndex(
    (item) => route.path === router.resolve(item.route).path
  );
});

watch(
  route,
  () => {
    active.value = items.value.findIndex(
      (item) => route.path === router.resolve(item.route).path
    );
  },
  { immediate: true }
);
</script>
<template>
  <div class="header">
    <div class="logo">
      <RouterLink to="/" class="link">Crypto Issues</RouterLink>
    </div>
    <div class="menu">
      <TabMenu :activeIndex="active" :model="items">
        <template #item="{ label, item, props }">
          <router-link v-if="item.route" v-slot="routerProps" :to="item.route" custom>
            <a :href="routerProps.href" v-bind="props.action" @click="($event) => routerProps.navigate($event)">
              <span v-bind="props.icon" />
              <span v-bind="props.label">{{ label }}</span>
            </a>
          </router-link>
        </template>
      </TabMenu>
    </div>
  </div>
</template>

<style scoped>
.link,
.link :visited {
  color: inherit;
  text-decoration: none;
  font-size: x-large;
  padding: 0.5rem;
  font-size: xx-large;
}

.menu {
  float: right;
}

.logo {
  float: left;
}

.header {
  height: 100px;
}
</style>