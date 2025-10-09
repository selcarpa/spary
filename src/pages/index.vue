<template>
  <v-card class="fill-height">
    <v-layout class="fill-height">
      <v-navigation-drawer
          expand-on-hover
          permanent
          rail
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
          <v-list-item prepend-icon="mdi-just-nothing" title="🌊" value="spary"></v-list-item>
          <v-list-item prepend-icon="mdi-cog" title="Settings" value="settings"></v-list-item>
        </v-list>
      </v-navigation-drawer>

      <v-main style="height: 100vh">
        <Spary />
      </v-main>
    </v-layout>
  </v-card>
</template>

<script lang="ts" setup>
  //
</script>
