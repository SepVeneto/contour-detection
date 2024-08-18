<template>
  <div>
    <button
      class=" bg-indigo-500 text-white py-2 px-5 rounded"
      @click="handleUpload"
    >上传文件</button>
    <input
      ref="fileRef"
      class=" hidden"
      type="file"
      @change="onUpload"
    />
  </div>
</template>

<script setup>
import { ref } from 'vue'
const imgurl = defineModel({ type: String })
const fileRef = ref()
function handleUpload() {
  fileRef.value.click()
}

async function onUpload(evt) {
  const file = evt.target.files[0]

  const url = await readFile(file)
  console.log('url', url)
  imgurl.value = url
  return

  // const img = await loadImage(url)

  const canvas = document.createElement('canvas')
  const ctx = canvas.getContext('2d')
  canvas.width = img.width
  canvas.height = img.height
  ctx.drawImage(img, 0, 0)

  // const imageData = ctx.getImageData(0, 0, img.width, img.height)
  // console.log('imageData', imageData)
  // const processed = process_img(imageData.data, img.width, img.height)
  // console.log('processed', processed)
  // const newImageData = new ImageData(new Uint8ClampedArray(processed), img.width, img.height)
  // canvas.width = img.width
  // canvas.height = img.height
  // ctx.putImageData(newImageData, 0, 0)

  newImage.value = canvas.toDataURL()
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
