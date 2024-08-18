<template>
  <div class="flex p-10">
    <div
      v-for="(item, index) in tags"
      class="bg-indigo-100 text-indigo-600 rounded border border-indigo-600 px-2 h-6 inline-flex content-center text-sm cursor-pointer"
      :key="index"
    >{{ item.label }}</div>
  </div>
</template>

<script setup>
const tags = [
  { label: '姓名', value: 'name' },
]
</script>
