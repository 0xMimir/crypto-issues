<script setup lang="ts">
import { useCryptocurrenciesStore } from "@/store/index";
import type { LanguageCount } from "@/types/languageCount";

import { ref } from "vue";

const store = useCryptocurrenciesStore();

const languages = ref<any>(null);

store.getLanguageCounts().then((response) => {
  response = formatData(response);
  languages.value = {
    labels: response.map((row) => row.language),
    datasets: [
      {
        label: "Count",
        data: response.map((row) => row.count),
      },
    ],
  };
});

function formatData(languages: LanguageCount[]): LanguageCount[] {
  const total = languages.reduce((sum, language) => sum + language.count, 0);
  languages = languages.filter((language) => language.count / total > 0.01);
  const rest =
    total - languages.reduce((sum, language) => sum + language.count, 0);

  languages.push({
    language: "Rest",
    count: rest,
  } as LanguageCount);

  return languages;
}
</script>
<template>
  <div>
    <h1>Statistics</h1>
    <div v-if="languages">
      <h2>Languages used</h2>
      <Chart type="pie" :data="languages" style="width: 50%" />
    </div>
  </div>
</template>