<script setup lang="ts">
defineProps({
  colorTheme: {
    type: String,
    required: false,
    default: "primary",
  },
  Disable: {
    type: Boolean,
    default: false,
  },
});
</script>

<template>
  <button
    :disabled="Disable"
    :class="[
      'defaultButton disabled:hover:border-gray-200 hover:bg-gray-300 hover:border-gray-300 hover:text-black',
      colorTheme == 'primary'
        ? ' disabled:hover:bg-gray-50 text-gray-600 bg-gray-50'
        : ' disabled:hover:bg-gray-500 text-gray-900 bg-gray-400',
    ]"
  >
    <div class="flex items-center w-full h-full justify-center">
      <slot></slot>
    </div>
  </button>
</template>
