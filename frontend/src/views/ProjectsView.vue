<script setup lang="ts">
import { useCryptocurrenciesStore } from '@/store';
import type { SearchGithubProject, SearchGithubProjectParams } from '@/types/searchGithubProject';
import { onMounted, ref, watch } from 'vue';


const store = useCryptocurrenciesStore();
const perPage = ref(10);
const page = ref(0);
const totalRecords = ref(0);
const languages = ref<any[] | undefined>(undefined);

const allLanguages = ref<any[]>([]);

const projects = ref<SearchGithubProject[]>([]);

function search() {
    const params = {
        page: page.value,
        perPage: perPage.value,
        languagesUsed: undefined
    } as SearchGithubProjectParams;

    if (languages.value && languages.value.length) {
        params.languagesUsed = languages.value.map(language => language.name);
    }

    store.searchProjects(params).then(response => {
        projects.value = response.data;
        totalRecords.value = response.totalItems;
    })
}

function changePage(pageState: { page: number }) {
    page.value = pageState.page;
}

onMounted(() => {
    store.getLanguageCounts().then(languagesResponse => {
        allLanguages.value = languagesResponse.map((language) => {
            return { name: language.language }
        });
    });

    search()
});

function changeLanguages() {
    page.value = 0;
}


watch([page, perPage, languages], search);

</script>
<template>
    <div>
        <h1>
            Projects
        </h1>
        <div>
            <MultiSelect v-model="languages" v-on:update:model-value="changeLanguages" :options="allLanguages"
                optionLabel="name" placeholder="Select Languages" filter :showToggleAll="false" display="chip" />
        </div>
        <br />
        <div v-if="projects.length">
            <DataTable :value="projects">
                <Column field="name" header="Name">
                    <template #body="slotProps">
                        <a class="link" :href="`https://github.com/${slotProps.data.name}`" rel="noopener">{{
                            slotProps.data.name
                        }}</a>
                    </template>
                </Column>
                <Column field="languagesUsed" header="Languages Used">
                    <template #body="slotProps">
                        {{ 
                            slotProps.data.languagesUsed.length 
                                ? slotProps.data.languagesUsed?.reduce((a: string, b: string) => `${a}, ${b}`)
                                : "Unknown"
                        }}
                    </template>
                </Column>
                <Column field="repositories" header="Repositories"></Column>
                <Column field="issues" header="Total Issues"></Column>
                <Column field="stargazersCount" header="Stargazers Count"></Column>
            </DataTable>
            <Paginator :rows="perPage" :totalRecords="totalRecords" :page="page" @page="changePage"></Paginator>
        </div>
        <div v-else class="spinner">
            <ProgressSpinner />
        </div>
    </div>
</template>