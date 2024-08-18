import { shallowRef, onMounted, reactive, ref } from 'vue'

function noop() {}

export function useDraggable(dom, onDragEnd = noop) {
  const dragging = ref(false)
  const start = reactive({
    x: 0,
    y: 0
  })
  const delta = reactive({
    x: 0,
    y: 0,
  })
  const current = reactive({
    x: 0,
    y: 0,
  })
  const rect = shallowRef({})
  const offset = reactive({
    x: 0,
    y: 0,
  })

  onMounted(() => {
    dom.value.addEventListener('mousedown', handleMousedown)
  })

  function handleMousedown(evt) {
    rect.value = evt.target.getBoundingClientRect()
    dragging.value = true
    const { clientX, clientY } = evt
    start.x = clientX - rect.value.x
    start.y = clientY - rect.value.y
    current.x = clientX - rect.value.x
    current.y = clientY - rect.value.y
    offset.x = 0
    offset.y = 0


    dom.value.addEventListener('mousemove', handleMousemove)
    dom.value.addEventListener('mouseup', handleMouseup)
  }
  function handleMouseup(evt) {
    dragging.value = false
    dom.value.removeEventListener('mousemove', handleMousemove)
    dom.value.removeEventListener('mouseup', handleMouseup)

    onDragEnd({ start, current })

    start.x = 0
    start.y = 0
    current.x = 0
    current.y = 0
    offset.x = 0
    offset.y = 0
  }
  function handleMousemove(evt) {
    const { clientX, clientY } = evt
    const { x, y } = rect.value
    delta.x = (clientX - x) - current.X
    delta.y = (clientY - y) - current.y

    current.x = (clientX - x)
    current.y = (clientY - y)

    offset.x = current.x - start.x
    offset.y = current.y - start.y

    if (offset.x < 0) {
      start.x -= delta.X
      // offset.x = Math.abs(offset.x)
    }
    if (offset.y < 0) {
      start.y -= delta.Y
      // offset.y = Math.abs(offset.y)
    }
  }

  return {
    start,
    current,
    offset,
  }
}