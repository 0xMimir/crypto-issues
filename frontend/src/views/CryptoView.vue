<script setup lang="ts">
import { useCryptocurrenciesStore } from "@/store";
import type { CryptoCurrencyWithRepositories } from "@/types/cryptoCurrencyWithRepositories";
import { ref } from "vue";
import { useRoute } from "vue-router";

const store = useCryptocurrenciesStore();
const cryptoId = useRoute().params.id as string;
const crypto = ref<CryptoCurrencyWithRepositories | null>(null);

store.getCryptocurrency(cryptoId).then((response) => (crypto.value = response));
</script>
<template>
  <div v-if="crypto">
    <div>
      <h1>{{ crypto.name }}</h1>
      <p>
        Coingecko:
        <a
          class="link"
          :href="`https://coingecko.com/en/coins/${crypto.coingeckoId}`"
          rel="noopener"
          >{{ crypto.coingeckoId }}
        </a>
      </p>
      <p>
        Github:
        <a
          class="link"
          :href="`https://github.com/${crypto.github}`"
          rel="noopener"
          >{{ crypto.github }}
        </a>
      </p>
    </div>
    <div>
      <h3>Project repositories</h3>
      <DataTable :value="crypto.repositories">
        <Column field="repositoryName" header="Repository name">
          <template #body="slotProps">
            <RouterLink class="link" :to="`/repository/${slotProps.data.id}`">
              {{ slotProps.data.repositoryName }}
            </RouterLink>
          </template>
        </Column>
        <Column field="github" header="Github">
          <template #body="slotProps">
            <a
              class="link"
              :href="`https://github.com/${crypto.github}/${slotProps.data.repositoryName}`"
              rel="noopener"
              >github</a
            >
          </template>
        </Column>
      </DataTable>
    </div>
  </div>
  <div v-else>Loading</div>
</template>
