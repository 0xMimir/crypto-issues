<script setup lang="ts">
import { formatDate } from '@/composables/time';
import { useCryptocurrenciesStore } from '@/store';
import type { SearchRepository } from '@/types/searchRepository';
import { onMounted, ref, watch } from 'vue';

const store = useCryptocurrenciesStore();

const repositories = ref<SearchRepository[]>([]);
const perPage = ref(50);
const page = ref(0);
const totalRecords = ref(0);
const archived = ref<any | undefined>(undefined);
const fork = ref<any | undefined>(undefined);
const language = ref<any | undefined>(undefined);
const languages = ref<any[]>([]);

const archivedOptions = ref([
    { name: "Active", code: false },
    { name: "Archived", code: true },
])

const forkOptions = ref([
    { name: "Original", code: false },
    { name: "Fork", code: true },
])

function search() {
    store.searchRepositories({
        page: page.value,
        perPage: perPage.value,
        archived: archived.value?.code,
        fork: fork.value?.code,
        language: language.value?.name
    }).then(response => {
        repositories.value = response.data;
        totalRecords.value = response.totalItems - 1;
    })
}

function changePage(pageState: { page: number }) {
    page.value = pageState.page;
}

onMounted(() => {
    store.getLanguageCounts().then((response) => {
        languages.value = response.map(languageCount => {
            return { name: languageCount.language }
        });
    })

    search()
});

watch([perPage, page, archived, fork, language], search);
</script>

<template>
    <div>
        <h1>Repositories</h1>
        <div>
            <Dropdown v-model="archived" :options="archivedOptions" optionLabel="name" placeholder="Status" showClear
                class="dropdown" />
            <Dropdown v-model="fork" :options="forkOptions" optionLabel="name" placeholder="Show forks" showClear
                class="dropdown" />
            <Dropdown v-model="language" :options="languages" filter optionLabel="name" placeholder="Filter by Language"
                showClear class="dropdown" />
        </div>
        <br>
        <div v-if="repositories.length">

            <DataTable :value="repositories" showGridlines>
                <Column field="repositoryName" header="Name">
                    <template #body="slotProps">
                        <a class="link" :href="slotProps.data.url">
                            {{ slotProps.data.project }}/{{ slotProps.data.repositoryName }}
                        </a>
                    </template>
                </Column>
                <Column field="language" header="Language"></Column>
                <Column field="stargazersCount" header="Stargazers"></Column>
                <Column field="createdAt" header="Created At">
                    <template #body="slotProps">
                        {{ formatDate(new Date(slotProps.data.createdAt)) }}
                    </template>
                </Column>
                <Column field="updatedAt" header="Updated At">
                    <template #body="slotProps">
                        {{ formatDate(new Date(slotProps.data.updatedAt)) }}
                    </template>
                </Column>
                <Column field="forksCount" header="Forks"></Column>
                <Column field="fork" header="Fork"></Column>
                <Column field="archived" header="Archived"></Column>
            </DataTable>
            <Paginator :rows="perPage" :totalRecords="totalRecords" :page="page" @page="changePage"></Paginator>
        </div>
        <div v-else class="spinner">
            <ProgressSpinner />
        </div>
    </div>
</template>
<style scoped>
.dropdown {
    margin: 5px;
}
</style>