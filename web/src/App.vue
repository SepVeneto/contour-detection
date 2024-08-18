<template>
  <div class="flex">
    <TagList />

    <ImageEditor v-model="newImage" />

    <ImageUpload v-model="newImage" />

  </div>
</template>

<script setup>
import TagList from './components/tagList.vue';
import ImageUpload from './components/ImageUpload.vue';

import init, { process_img } from 'core'
import { onMounted, ref } from 'vue'
import ImageEditor from './components/ImageEditor.vue';
initEnv()
async function initEnv() {
  await init()
}
const newImage = ref('')

</script>
