<template>
  <v-app>
    <notification-provider />
    <v-main>
      <v-card class="fill-height">
        <v-layout class="fill-height">
          <v-navigation-drawer
              expand-on-hover
              permanent
              rail
              v-model="drawer"
          >
            <v-list>
              <v-list-item
                  prepend-avatar="/src/assets/logo.svg"
                  subtitle=""
                  title="Spary"
              ></v-list-item>
            </v-list>

            <v-divider></v-divider>

            <v-list density="compact" nav>
              <v-list-item
                prepend-icon="mdi-just-nothing"
                title="🌊"
                value="spary"
                @click="router.push('/')"
              ></v-list-item>
              <v-list-item
                  prepend-icon="mdi-airport"
                  title="Nodes"
                  value="nodes"
                  @click="router.push('/nodes')"
              ></v-list-item>
              <v-list-item
                  prepend-icon="mdi-cog"
                  title="Settings"
                  value="settings"
                  @click="router.push('/settings')"
              ></v-list-item>
            </v-list>
          </v-navigation-drawer>

          <v-main style="height: 100vh">
            <router-view/>
          </v-main>
        </v-layout>
      </v-card>
    </v-main>
  </v-app>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import notificationProvider from './components/notify/notificationProvider.vue'

const drawer = ref(true)
const router = useRouter()
</script>
