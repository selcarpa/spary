<template>
  <v-container>
    <v-card class="mx-auto" max-width="600">
      <v-card-title>{{ $t('app.settings') }}</v-card-title>
      <v-card-text>
        <v-row>
          <v-col cols="12">
            <v-btn @click="toggleDarkMode" disabled>
              {{ darkMode ? $t('common.darkModeOff') : $t('common.darkModeOn') }}
            </v-btn>
          </v-col>
          <v-col cols="12">
            <h3>{{ $t('common.language') }}</h3>
            <v-btn-toggle v-model="selectedLanguage" mandatory>
              <v-btn value="en">{{ $t('common.english') }}</v-btn>
              <v-btn value="zh">{{ $t('common.chinese') }}</v-btn>
            </v-btn-toggle>
          </v-col>
        </v-row>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang="ts" setup>
import { ref, watch, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';

const { locale } = useI18n();
const selectedLanguage = ref(locale.value);
const darkMode = ref(false);

// Update the translation when the language changes
watch(selectedLanguage, (newLang) => {
  locale.value = newLang;
  localStorage.setItem('language', newLang);
});

// Initialize the selected language from localStorage
onMounted(() => {
  const savedLanguage = localStorage.getItem('language');
  if (savedLanguage) {
    selectedLanguage.value = savedLanguage;
    locale.value = savedLanguage;
  }
});

// Toggle dark mode
const toggleDarkMode = () => {
  darkMode.value = !darkMode.value;
  // Here you would typically update a theme preference
};
</script>
