<script setup lang="ts">
defineProps<{
  label: string;
  value?: string | number;
  valueClass?: string;
}>();
</script>

<template>
  <div class="flex flex-col items-center p-6 bg-white rounded-xl shadow-sm hover:shadow-lg hover:scale-[1.03] transition-all duration-300 cursor-default">
    <div class="text-xs text-gray-400 uppercase tracking-widest mb-3">
      {{ label }}
    </div>
    <div class="text-4xl font-bold text-gray-800" :class="valueClass">
      <slot>
        {{ value }}
      </slot>
    </div>
  </div>
</template>
