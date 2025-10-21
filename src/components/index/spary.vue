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
import {genConfigFile, removeConfigFile} from "@/utils/bootArgs.ts";

const functionStatus = ref<String>("Off");

function toggleFunctionStatus() {
  functionStatus.value = functionStatus.value === "Off" ? "On" : "Off";
  spary_switch(functionStatus.value === "On")

  if (functionStatus.value === "On") {
    genConfigFile()
  } else {
    removeConfigFile()
  }
}

async function spary_switch(status: boolean) {
  await invoke("spary_switch", {status})
}
</script>
