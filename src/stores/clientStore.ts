import { defineStore } from "pinia";

export const useClientStore = defineStore("ClientStore", {
  state: () => {
    return {
      clients: [],
      client: null,
    };
  },
  actions: {
    getAllClients: async function () {
    },
    getOneClient: async function (id: number) {},
    createOneClient: async function () {},
    deleteOneClient: async function (id: number) {},
    updateOneClient: async function (id: number) {},
  },
});
