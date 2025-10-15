<template>
  <v-menu>
    <template v-slot:activator="{ props }">
      <v-btn
        v-bind="props"
        icon
      >
        <v-icon>mdi-translate</v-icon>
      </v-btn>
    </template>
    <v-list>
      <v-list-item
        v-for="(item, i) in languages"
        :key="i"
        @click="changeLanguage(item.code)"
        :active="currentLanguage === item.code"
      >
        <v-list-item-title>{{ item.name }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

const { locale } = useI18n();

const languages = [
  { code: 'en', name: 'English' },
  { code: 'zh', name: '中文' }
];

const currentLanguage = computed(() => locale.value);

const changeLanguage = (lang: string) => {
  locale.value = lang;
  // Store the selected language in localStorage so it persists across sessions
  localStorage.setItem('language', lang);
};

// Set the initial language from localStorage if available
const savedLanguage = localStorage.getItem('language');
if (savedLanguage && languages.some(lang => lang.code === savedLanguage)) {
  locale.value = savedLanguage;
}
</script>