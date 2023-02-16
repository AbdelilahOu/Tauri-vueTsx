<script setup lang="ts">
import { RouterView } from "vue-router";
import { Transition } from "vue";
</script>

<template>
  <div class="w-screen overflow-x-hidden h-screen bg-white">
    <RouterView v-slot="{ Component }">
      <Transition name="router" :appear="true">
        <component :is="Component" />
      </Transition>
    </RouterView>
  </div>
</template>
