import { createRouter, createWebHistory } from "vue-router";
import { IndexView } from "@/views/content/IndexView";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "Index",
      component: IndexView,
    },
  ],
});

export default router;