<template>
  <div>
    <v-snackbar
        v-for="(item, index) in notifyQueue"
        :key="item.id"
        v-model="visible[item.id]"
        :timeout="item.timeout"
        :color="item.color"
        :variant="item.variant"
        top
        right
        :style="{
        zIndex: 9999,
        marginTop: `${index * 70 + 16}px`  // 每条通知向下偏移
      }"
    >
      {{ item.message }}
    </v-snackbar>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue'
import { notifyQueue } from './notifyStore'

const visible: Record<number, boolean> = reactive({})

watch(
    () => notifyQueue.slice(),
    (queue) => {
      queue.forEach(item => {
        if (visible[item.id] === undefined) visible[item.id] = true
      })
    },
    { immediate: true }
)
</script>
