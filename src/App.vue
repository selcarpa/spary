<template>
  <v-app>
    <notification-provider/>
    <v-main>
      <v-card class="fill-height">
        <v-layout class="fill-height">
          <main-drawer/>
          <v-main style="height: 100vh">
            <router-view/>
          </v-main>
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
