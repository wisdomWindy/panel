<script setup lang="ts">
import { reactive } from 'vue';

const form = reactive({
  ownerEnabled: true,
  ownerTemplate: '欢迎 {username} 主播！',
  ownerCooldown: 300,
  adminEnabled: true,
  adminTemplate: '房管 {username} 来了！',
  adminCooldown: 120,
  captainEnabled: true,
  captainTemplate: '感谢舰长 {username} 的支持！',
  captainCooldown: 60
});

function getValues() {
  return { ...form };
}

defineExpose({ getValues });
</script>

<template>
  <el-form :model="form" label-width="150px">
    <el-divider content-position="left">
      主播回复
    </el-divider>
    <el-form-item label="启用">
      <el-switch v-model="form.ownerEnabled" />
    </el-form-item>
    <el-form-item label="回复模板">
      <el-input v-model="form.ownerTemplate" placeholder="支持 {username}, {userType} 变量" />
    </el-form-item>
    <el-form-item label="冷却时间">
      <el-input-number
        v-model="form.ownerCooldown"
        :min="10"
        :max="3600"
        :step="10"
      />
      <span class="ml-2">秒</span>
    </el-form-item>

    <el-divider content-position="left">
      房管回复
    </el-divider>
    <el-form-item label="启用">
      <el-switch v-model="form.adminEnabled" />
    </el-form-item>
    <el-form-item label="回复模板">
      <el-input v-model="form.adminTemplate" placeholder="支持 {username}, {userType} 变量" />
    </el-form-item>
    <el-form-item label="冷却时间">
      <el-input-number
        v-model="form.adminCooldown"
        :min="10"
        :max="3600"
        :step="10"
      />
      <span class="ml-2">秒</span>
    </el-form-item>

    <el-divider content-position="left">
      舰长回复
    </el-divider>
    <el-form-item label="启用">
      <el-switch v-model="form.captainEnabled" />
    </el-form-item>
    <el-form-item label="回复模板">
      <el-input v-model="form.captainTemplate" placeholder="支持 {username}, {userType} 变量" />
    </el-form-item>
    <el-form-item label="冷却时间">
      <el-input-number
        v-model="form.captainCooldown"
        :min="10"
        :max="3600"
        :step="10"
      />
      <span class="ml-2">秒</span>
    </el-form-item>
  </el-form>
</template>
