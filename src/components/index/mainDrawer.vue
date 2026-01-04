<script setup lang="ts">

import {ref} from 'vue'
import {useRouter} from 'vue-router'
import LanguageSwitcher from '../LanguageSwitcher.vue'
import {useI18n} from "vue-i18n";

const drawer = ref(true)
const rail = ref(true)
const router = useRouter()

useI18n()
</script>

<template>
  <v-navigation-drawer
      expand-on-hover
      permanent
      :rail="rail"
      v-model="drawer"
  >
    <v-list>
      <v-list-item
          prepend-avatar="/src/assets/logo.svg"
          subtitle=""
          :title="$t('app.title')"
          @click="rail = !rail"
      ></v-list-item>
    </v-list>

    <v-divider></v-divider>

    <v-list density="compact" nav>
      <v-list-item
          prepend-icon="mdi-home"
          :title="$t('app.mainPage')"
          value="spary"
          @click="router.push('/')"
      ></v-list-item>
      <v-list-item
          prepend-icon="mdi-airport"
          :title="$t('app.nodes')"
          value="nodes"
          @click="router.push('/nodes')"
      ></v-list-item>
      <v-list-item
          prepend-icon="mdi-cog"
          :title="$t('app.settings')"
          value="settings"
          @click="router.push('/settings')"
      >
        <template v-slot:append>
          <language-switcher/>
        </template>
      </v-list-item>
      <v-list-item
          prepend-icon="mdi-bug"
          :title="$t('app.debug')"
          value="coreLog"
          @click="router.push('/coreLog')"
      ></v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<style scoped>

</style>