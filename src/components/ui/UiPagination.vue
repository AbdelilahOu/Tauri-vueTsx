<script setup lang="ts">
const props = defineProps({
  page: {
    type: Number,
    required: true,
  },
  itemsNumber: {
    type: Number,
    required: true,
  },
});
const emits = defineEmits(["goBack", "goForward"]);

const goBack = () => {
  if (props.page > 0) {
    emits("goBack");
  }
};
const goForward = () => {
  if (props.page + 1 < Math.ceil(props.itemsNumber / 15)) {
    emits("goForward");
  }
};
</script>

<template>
  <div class="h-8 w-full mt-2 flex items-center justify-center">
    <div
      class="h-fit w-fit flex items-center text-lg bg-slate-200 px-4 rounded-sm font-semibold text-gray-800 gap-4"
    >
      <span
        class="rounded-full flex items-center justify-center cursor-pointer"
        @click="goBack"
      >
        -
      </span>
      <div class="flex w-full h-full items-center">
        <span class="px-1 text-base text-gray-400">
          {{ props.page !== 0 && props.page }}
        </span>
        <span class="px-1">{props.page + 1}</span>
        <span class="px-1 text-base text-gray-400">
          {{
            props.page + 1 !== Math.ceil(props.itemsNumber / 15) &&
            props.page + 2
          }}
        </span>
      </div>
      <span
        @click="goForward"
        class="rounded-full flex items-center justify-center cursor-pointer"
      >
        +
      </span>
    </div>
  </div>
</template>
