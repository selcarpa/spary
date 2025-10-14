<script setup lang="ts">
import {useRoute} from 'vue-router/auto'

const route = useRoute('/addNode/[groupId]')
// 🔥 类型会自动推断出 route.params.groupId 是 string

const preSelectGroupId = route.params.groupId
console.log(`preSelectGroupId:${preSelectGroupId}`)
</script>

<template>
  <add-node :group-id="preSelectGroupId"/>
</template>