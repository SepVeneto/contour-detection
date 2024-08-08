<template>
  <input type="file" @change="onUpload" />
  <img :src="newImage" />
</template>

<script setup>
import init, { process_img } from 'core'
import { onMounted, ref } from 'vue'
initEnv()
async function initEnv() {
  await init()
}
const newImage = ref('')

async function onUpload(evt) {
  const file = evt.target.files[0]

  const url = await readFile(file)

  const img = await loadImage(url)

  const canvas = document.createElement('canvas')
  const ctx = canvas.getContext('2d')
  canvas.width = img.width
  canvas.height = img.height
  ctx.drawImage(img, 0, 0)

  const imageData = ctx.getImageData(0, 0, img.width, img.height)
  console.log('imageData', imageData)
  const processed = process_img(imageData.data, img.width, img.height)
  console.log('processed', processed)
  const newImageData = new ImageData(new Uint8ClampedArray(processed), img.width, img.height)
  canvas.width = img.width
  canvas.height = img.height
  ctx.putImageData(newImageData, 0, 0)

  newImage.value = canvas.toDataURL()
}

async function loadImage(src) {
  const { promise, resolve, reject } = Promise.withResolvers()
  const img = new Image()
  img.onload = () => resolve(img)
  img.onerror = reject
  img.src = src
  return promise
}
async function readFile(file) {
  const { promise, resolve, reject } = Promise.withResolvers()
  const reader = new FileReader()
  reader.onload = (evt) => resolve(evt.target.result)
  reader.onerror = reject
  reader.readAsDataURL(file)

  return promise
}
</script>
