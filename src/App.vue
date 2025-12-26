<template>
  <v-app>
    <notification-provider/>
    <v-main class="d-flex" style="height: 100vh;">
      <v-card class="d-flex flex-column" style="height: 100%; width: 100%;">
        <v-layout class="d-flex flex-column" style="height: 100%;">
          <main-drawer/>
          <v-card class="flex-grow-1 overflow-y-auto ma-4">
            <router-view/>
          </v-card>
        </v-layout>
      </v-card>
    </v-main>
  </v-app>
</template>

<script lang="ts" setup>
import {useI18n} from 'vue-i18n'
import notificationProvider from './components/notify/notificationProvider.vue'
import MainDrawer from "@/components/index/mainDrawer.vue";

useI18n()
</script>
