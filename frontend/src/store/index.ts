import type { CryptoCurrencyView } from "@/types/cryptoCurrencyView";
import type { Pagination } from "@/types/pagination";
import { defineStore } from "pinia";
import { client } from "./axios";

export const useCryptocurrenciesStore = defineStore("cryptocurrencies", {
  actions: {
    async getCryptocurrencies(page: number, perPage: number): Promise<Pagination<CryptoCurrencyView>> {
      const response = await client.get("/api/v1/crypto", {
        params: {
          page,
          perPage
        }
      });
      return response.data;
    },
  },
});
