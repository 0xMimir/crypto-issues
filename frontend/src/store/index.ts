import type { CryptoCurrencyWithRepositories } from "@/types/cryptoCurrencyWithRepositories";
import type { CryptoCurrencyView } from "@/types/cryptoCurrencyView";
import type { Pagination } from "@/types/pagination";
import { defineStore } from "pinia";
import { client } from "./axios";
import type { Issue } from "@/types/issues";
import type { RepositoryView } from "@/types/repositoryView";
import type { LanguageCount } from "@/types/languageCount";
import type { SearchRepository, SearchRepositoryParams } from "@/types/searchRepository";
import type { SearchGithubProject, SearchGithubProjectParams } from "@/types/searchGithubProject";

export const useCryptocurrenciesStore = defineStore("cryptocurrencies", {
	actions: {
		async getCryptocurrencies(
			page: number,
			perPage: number
		): Promise<Pagination<CryptoCurrencyView>> {
			const response = await client.get("/api/v1/crypto", {
				params: {
					page,
					perPage,
				},
			});
			return response.data;
		},

		async getCryptocurrency(id: string): Promise<CryptoCurrencyWithRepositories> {
			const response = await client.get(`/api/v1/crypto/${id}`);
			return response.data;
		},

		async getRepository(id: string): Promise<RepositoryView> {
			const response = await client.get(`/api/v1/repository/${id}`);
			return response.data;
		},

		async getIssues(repositoryId: string, page: number, perPage: number): Promise<Pagination<Issue>> {
			const response = await client.get(`/api/v1/repository/${repositoryId}/issues`, {
				params: {
					page,
					perPage
				}
			});
			return response.data;
		},

		async getLanguageCounts(): Promise<LanguageCount[]> {
			const response = await client.get("/api/v1/statistics/languages-count");
			return response.data;
		},

		async searchRepositories(params: SearchRepositoryParams): Promise<Pagination<SearchRepository>> {
			const response = await client.get("/api/v1/repository/search", {
				params: params
			});
			return response.data;
		},

		async searchProjects(params: SearchGithubProjectParams): Promise<Pagination<SearchGithubProject>> {
			const response = await client.get("/api/v1/projects", {
				params: {
					languagesUsed: params.languagesUsed?.reduce((a,b) => `${a},${b}`),
					page: params.page,
					perPage: params.perPage,
					order: params.order,
					orderBy: params.orderBy
				} as SearchGithubProjectParams,
			});
			return response.data;
		}
	},
});
