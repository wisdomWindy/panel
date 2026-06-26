<template>
  <el-icon :size="size" :color="color">
    <component :is="name" />
  </el-icon>
</template>

<script setup>
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

defineProps({
  name: {
    type: String,
    required: true
  },
  size: {
    type: [Number, String],
    default: 24
  },
  color: {
    type: String,
    default: ''
  }
})
</script>
