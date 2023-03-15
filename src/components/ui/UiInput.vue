<script setup lang="ts">
import { ref } from "vue";

defineProps({
  Type: {
    type: String,
    default: "text",
  },
  Disable: {
    type: Boolean,
    default: false,
  },
  PlaceHolder: {
    type: String,
  },
  IsEmpty: {
    type: Boolean,
    required: true,
  },
});
defineEmits(["OnInputChange"]);
const inputValue = ref<string | number>("");
</script>

<template>
  <div
    :class="[
      'flex flex-nowrap h-full items-center border-2 rounded-sm transition-all duration-200',
      IsEmpty && 'border-red-300 placeholder:text-red-300',
    ]"
  >
    <slot></slot>
    <input
      :class="[
        'defaultInput border-none',
        IsEmpty && 'placeholder:text-red-300',
      ]"
      @input="$emit('OnInputChange', inputValue)"
      :placeholder="PlaceHolder"
      v-model="inputValue"
      :disabled="Disable"
      :type="Type"
    />
  </div>
</template>
