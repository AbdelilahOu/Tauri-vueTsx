<script setup lang="ts">
import { ref } from "vue";

const emit = defineEmits(["onCheck"]);
const checked = ref<boolean>(false);
const toggleCheck = () => {
  checked.value = !checked.value;
  emit("onCheck", checked.value);
};
</script>

<template>
  <div
    @click="toggleCheck"
    :class="[
      checked
        ? 'w-4 h-4 border-2 cursor-pointer flex items-center justify-center relative transition-all transform duration-100 border-gray-400  rounded-sm bg-gray-300 text-white'
        : 'w-4 h-4 flex rounded-sm border-gray-400 cursor-pointer rounded-ms border-2 transition-all transform duration-150',
    ]"
  >
    <Transition>
      <span v-if="checked" class="text-white text-[10px] absolute">✔</span>
    </Transition>
  </div>
</template>
