<template>
  <v-container class="fill-height d-flex align-center justify-center">
    <div>
      <v-switch
          :label="functionStatus === 'On' ? $t('spary.functionStatus.on') : $t('spary.functionStatus.off')"
          :model-value="functionStatus === 'On'"
          @update:model-value="toggleFunctionStatus"
      ></v-switch>
    </div>
  </v-container>
</template>

<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {genConfigFile, removeConfigFile} from "@/utils/coreBootArgs.ts";
import {coreStart} from "@/utils/coreBoot.ts";

const functionStatus = ref<String>("Off");

async function toggleFunctionStatus() {
  functionStatus.value = functionStatus.value === "Off" ? "On" : "Off";
  await spary_switch(functionStatus.value === "On")

  if (functionStatus.value === "On") {
    await genConfigFile()
    await coreStart()
  } else {
    await removeConfigFile()
  }
}

async function spary_switch(status: boolean) {
  await invoke("spary_switch", {status})
}
</script>
