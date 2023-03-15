<script setup lang="ts">
defineProps({
  IsText: {
    type: Boolean,
    required: true,
  },
  LinkPath: {
    type: String,
    default: "/",
    required: true,
  },
  LinkIcon: {
    type: String,
    required: true,
  },
  LinkText: {
    type: String,
    required: true,
  },
});
</script>

<template>
  <RouterLink :to="LinkPath">
    <span
      class="w-full flex h-9 rounded-sm items-center py-1 px-2 hover:bg-gray-200 transition-all duration-300"
    >
      {{ LinkIcon }}
      <span
        v-if="IsText"
        class="text-[rgba(25,23,17,0.6)] ml-1 whitespace-nowrap"
      >
        {{ LinkText }}
      </span>
    </span>
  </RouterLink>
</template>
