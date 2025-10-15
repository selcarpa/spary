<script setup lang="ts">
import {useRouter} from "vue-router";
import { useI18n } from 'vue-i18n';
const router = useRouter()
useI18n()

</script>

<template>
  <div class="fab-fixed">
    <v-speed-dial location="left top" transition="slide-y-transition">
      <template v-slot:activator="{ props: activatorProps }">
        <v-fab v-bind="activatorProps" size="large" icon="mdi-plus"></v-fab>
      </template>
      <v-btn key="1" prepend-icon="mdi-group"
             @click="router.push('/addGroup')">
        <template v-slot:prepend>
          <v-icon color="success"></v-icon>
        </template>
        {{ $t('nodesFloatButton.addGroup') }}
      </v-btn>
      <v-btn key="2" prepend-icon="mdi-airplane-marker">
        <template v-slot:prepend>
          <v-icon color="success"></v-icon>
        </template>
        {{ $t('nodesFloatButton.addNode') }}
      </v-btn>
    </v-speed-dial>
  </div>
</template>

<style scoped>
.fab-fixed {
  position: fixed;
  bottom: 16px;
  right: 16px;
}
</style>