import type { CryptoCurrencyWithRepositories } from "@/types/cryptoCurrencyWithRepositories";
import type { CryptoCurrencyView } from "@/types/cryptoCurrencyView";
import type { Pagination } from "@/types/pagination";
import { defineStore } from "pinia";
import { client } from "./axios";

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
  },
});
