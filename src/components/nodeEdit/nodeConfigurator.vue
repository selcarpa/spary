<script setup lang="ts">
import { watch, ref, computed, toRaw} from 'vue';
import { ConfigurationSchema } from "@/utils/core/configurator/schema/schema.ts";
import SchemaField from './SchemaField.vue';

const props = defineProps<{
  configurationSchema: ConfigurationSchema,
  modelValue: object | null, // Allow null for initial state
}>();

const emit = defineEmits(['update:modelValue']);

// Use a ref instead of a computed property with setter to avoid proxy issues
const localConfig = ref<Record<string, any>>(props.modelValue || {});

// Watch for changes in the configurationSchema prop to reset internalConfig
watch(() => props.configurationSchema, () => {
  // Reset localConfig to an empty object when schema changes
  localConfig.value = {};
  emit('update:modelValue', {});
}, { immediate: true });

// Watch for changes in modelValue prop to update localConfig
watch(() => props.modelValue, (newVal) => {
  localConfig.value = newVal || {};
});

// Watch for changes in localConfig to emit updates
watch(localConfig, (newVal) => {
  emit('update:modelValue', newVal);
}, { deep: true });

// 使用toRaw来避免响应式代理问题
const mainSchema = computed(() => {
    const rawSchema = toRaw(props.configurationSchema);
    return rawSchema.schema;
});
</script>

<template>
  <v-card class="pa-4" variant="tonal">
    <div v-for="(field, key) in mainSchema.shape" :key="key.toString()">
      <SchemaField
          :schema="field"
          :label="key.toString()"
          :model-value="localConfig[key.toString()]"
          @update:modelValue="newValue => {
            const newConfig = { ...localConfig.value };
            newConfig[key.toString()] = newValue;
            localConfig.value = newConfig;
          }"
      />
    </div>
  </v-card>
</template>

<style scoped>
</style>