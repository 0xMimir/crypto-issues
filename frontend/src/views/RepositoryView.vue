<script setup lang="ts">
import { useCryptocurrenciesStore } from "@/store/index";
import { formatDate } from "@/composables/time";
import type { Issue } from "@/types/issues";
import type { RepositoryView } from "@/types/repositoryView";
import { ref } from "vue";
import { useRoute } from "vue-router";

const repositoryId = useRoute().params.id as string;

const repository = ref<RepositoryView | null>(null);
const issues = ref<Issue[]>([]);
const page = ref(0);
const perPage = ref(50);
const lastPage = ref(0);
const store = useCryptocurrenciesStore();

function loadData() {
  store.getIssues(repositoryId, page.value, perPage.value).then((response) => {
    issues.value = response.data;
    lastPage.value = response.totalItems;
  });
}

function changePage(pageState: { page: number }) {
  page.value = pageState.page + 1;
  loadData();
}

loadData();

store
  .getRepository(repositoryId)
  .then((response) => (repository.value = response));
</script>
<template>
  <div>
    <div v-if="repository">
      <h2>
        Project:
        <a class="link" :href="`https://github.com/${repository.project}`">{{
          repository.project
        }}</a>
      </h2>
      <h3>
        Repository:
        <a
          class="link"
          :href="`https://github.com/${repository.project}/${repository.name}`"
          >{{ repository.name }}</a
        >
      </h3>
    </div>
    <div v-if="issues.length !== 0 && repository">
      <DataTable :value="issues" showGridlines>
        <Column field="name" header="Name">
          <template #body="slotProps">
            <a
              class="link"
              :href="`https://github.com/${repository.project}/${repository.name}/issues/${slotProps.data.issue}`"
            >
              #{{ slotProps.data.issue }}
            </a>
          </template>
        </Column>
        <Column field="title" header="Title"></Column>
        <Column field="createdAt" header="Created At">
          <template #body="slotProps">
            {{ formatDate(new Date(slotProps.data.createdAt)) }}
          </template>
        </Column>
      </DataTable>
      <Paginator
        :rows="perPage"
        :totalRecords="lastPage"
        :page="page"
        @page="changePage"
      ></Paginator>
    </div>
  </div>
</template>
<style scoped>
.p-paginator-page,
.p-paginator-prev,
.p-paginator-next,
.p-paginator-last,
.p-paginator-first {
  font-size: larger;
  color: var(--vt-c-text-dark-2);
}

.p-paginator-page {
  margin: 0.5rem;
}
</style>
