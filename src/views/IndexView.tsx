import { defineComponent } from "vue";

export const IndexView = defineComponent({
  name: "Index",
  components: {},
  setup() {
    return () => (
      <main class="w-full h-full flex items-center justify-center">
        <h1>Welcome To Sarter project</h1>
      </main>
    );
  },
});
