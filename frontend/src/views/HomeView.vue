<script setup lang="ts">
import { useCryptocurrenciesStore } from "@/store/index";
import type { CryptoCurrencyView } from "@/types/cryptoCurrencyView";
import { ref } from "vue";

const cryptoCurrencies = ref<CryptoCurrencyView[]>([]);
const page = ref(0);
const perPage = ref(100);
const lastPage = ref(0);
const store = useCryptocurrenciesStore();

function loadData() {
  store.getCryptocurrencies(page.value, perPage.value).then((response) => {
    cryptoCurrencies.value = response.data;
    lastPage.value = response.lastPage + 1;
  });
}

function changePage(pageState: { page: number; }) {
  page.value = pageState.page;
  loadData();
}

loadData();
</script>
<template>
  <div v-if="cryptoCurrencies.length !== 0">
    <DataTable :value="cryptoCurrencies" showGridlines>
      <Column field="name" header="Name">
        <template #body="slotProps">
          <RouterLink class="link" :to="`/crypto/${slotProps.data.id}`">
            {{ slotProps.data.name }}
          </RouterLink>
        </template>
      </Column>
      <Column field="coingeckoId" header="Coingecko Id"></Column>
      <Column field="github" header="Github">
        <template #body="slotProps">
          <a
            class="link"
            :href="`https://github.com/${slotProps.data.github}`"
            rel="noopener"
            >{{ slotProps.data.github }}</a
          >
        </template>
      </Column>
      <Column header="Repositories">
        <template #body="slotProps">
          <td>{{ slotProps.data.repositories.length }}</td>
        </template>
      </Column>
      <Column field="issues" header="Issues"></Column>
    </DataTable>
    <Paginator
      :rows="perPage"
      :totalRecords="lastPage"
      @page="changePage"
    ></Paginator>
  </div>
</template>
<style>
.link,
.link :visited {
  color: inherit;
  text-decoration: none;
}
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
