<template>
  <div class="relative">
    <FreeScene>
      <canvas ref="canvasRef" />
      <div
        class="absolute top-0 left-0 bg-indigo-300 opacity-15 pointer-events-none"
        ref="maskRef"
        :style="maskStyle"
      ></div>

      <FreeDom
        v-for="(area, index) in selectorAreas"
        v-model="selectorAreas[index]"
        class="absolute bg-indigo-300 opacity-15"
        :style="genStyle(area)"
      >
      </FreeDom>
    </FreeScene>
  </div>
</template>

<script setup>
import { ref, watchEffect, shallowRef, onMounted, computed, triggerRef } from 'vue'
import { useDraggable } from '@/hooks/useDraggable';
import { FreeScene, FreeDom } from '@sepveneto/free-dom'
import '@sepveneto/free-dom/css'

const canvasRef = ref()
const maskRef = ref()
const imgurl = defineModel({ type: String })
const img = shallowRef()

const { promise, resolve } = Promise.withResolvers()
const canvas = shallowRef(promise)

const selectorAreas = shallowRef([])

const draggable = useDraggable(canvasRef, ({ start, current }) => {
  selectorAreas.value.push({
    startX: start.x,
    startY: start.y,
    endX: current.x,
    endY: current.y,
  })
  triggerRef(selectorAreas)
})
const maskStyle = computed(() => {
  return {
    top: draggable.start.y + 'px',
    left: draggable.start.x + 'px',
    width: draggable.offset.x + 'px',
    height: draggable.offset.y + 'px',
  }
})

watchEffect(async () => {
  if (!imgurl.value) return

  console.log(imgurl.value)
  const image = await loadImage(imgurl.value)
  img.value = image

  const _canvas = await canvas.value
  const ctx = _canvas.getContext('2d')
  _canvas.width = img.value.width
  _canvas.height = img.value.height
  ctx.drawImage(img.value, 0, 0)
})

onMounted(() => {
  resolve(canvasRef.value)
})


async function loadImage(src) {
  const { promise, resolve, reject } = Promise.withResolvers()
  const img = new Image()
  img.onload = () => resolve(img)
  img.onerror = reject
  img.src = src
  return promise
}
function genStyle(style) {
  return {
    top: style.startY + 'px',
    left: style.startX + 'px',
    width: style.endX - style.startX + 'px',
    height: style.endY - style.startY + 'px',
  }
}
</script>
