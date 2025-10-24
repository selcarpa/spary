<script setup lang="ts">
import { Message, Terminal } from 'vue-web-terminal';

const initLog: Message[] = [
  {
    content: 'Spary.S > '
  }
]

</script>

<template>
    <terminal
      name="main-console"
      theme="dark"
      :show-header="false"
      :enable-default-command="false"
      cursor-style="block"
      context=""
      context-suffix=""
      :init-log="initLog"
    />
</template>

<style scoped>

</style>
