<script setup lang="ts">
import {computed, toRaw} from 'vue';
import { z, ZodObject, ZodString, ZodNumber, ZodBoolean, ZodEnum, ZodOptional } from 'zod';

defineOptions({
  name: 'SchemaField'
})

const props = defineProps<{
  schema: z.ZodTypeAny;
  modelValue: any;
  label: string;
}>();

// 使用toRaw获取原始schema对象，避免Proxy问题
const rawSchema = toRaw(props.schema);

const emit = defineEmits(['update:modelValue']);
const isOptional = computed(() => rawSchema.optional());

const value = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val),
});

const typeName = computed(() => {
  if (rawSchema instanceof ZodObject) return 'ZodObject';
  if (rawSchema instanceof ZodString) return 'ZodString';
  if (rawSchema instanceof ZodNumber) return 'ZodNumber';
  if (rawSchema instanceof ZodBoolean) return 'ZodBoolean';
  if (rawSchema instanceof ZodEnum) return 'ZodEnum';
  if (rawSchema instanceof ZodOptional) return 'ZodOptional';
  // Fallback for unknown types or types not handled explicitly
  return 'Unknown';
});

const objectValue = computed(() => value.value as Record<string, any> | null);

const objectSchema = computed(() => {
  const schema = rawSchema;
  if (schema instanceof ZodOptional) {
    const unwrapped = schema.unwrap();
    if (unwrapped instanceof ZodObject) {
      return unwrapped;
    }
  }
  if (schema instanceof ZodObject) {
    return schema;
  }
  return null;
});

</script>

<template>
  <div class="schema-field">
    <template v-if="typeName === 'ZodString'">
      <v-text-field
          v-model="value"
          :label="label"
          :hint="isOptional ? 'Optional' : ''"
          persistent-hint
      ></v-text-field>
    </template>

    <template v-else-if="typeName === 'ZodNumber'">
      <v-text-field
          v-model.number="value"
          type="number"
          :label="label"
          :hint="isOptional ? 'Optional' : ''"
          persistent-hint
      ></v-text-field>
    </template>

    <template v-else-if="typeName === 'ZodBoolean'">
      <v-switch
          v-model="value"
          :label="label"
          color="primary"
          :hint="isOptional ? 'Optional' : ''"
          persistent-hint
      ></v-switch>
    </template>

    <template v-else-if="typeName === 'ZodEnum'">
      <v-select
          v-model="value"
          :items="(rawSchema as ZodEnum<any>).options"
          :label="label"
          :hint="isOptional ? 'Optional' : ''"
          persistent-hint
      ></v-select>
    </template>

    <template v-else-if="objectSchema">
      <v-card variant="outlined" class="pa-4 mb-4">
        <div class="text-subtitle-1 font-weight-medium">{{ label }}</div>
        <div v-for="(field, key) in objectSchema.shape" :key="key">
          <SchemaField
              :schema="field"
              :label="key.toString()"
              :model-value="objectValue ? objectValue[key] : undefined"
              @update:modelValue="newValue => {
                const newObjectValue = { ...value.value };
                newObjectValue[key] = newValue;
                value.value = newObjectValue;
              }"
          />
        </div>
      </v-card>
    </template>

    <template v-else-if="typeName === 'ZodOptional'">
      <SchemaField
          :schema="(rawSchema as ZodOptional<any>).unwrap()"
          v-model="value"
          :label="label"
      />
    </template>

    <template v-else>
      <p class="text-caption">Unsupported type: {{ typeName }} for {{label}}</p>
    </template>
  </div>
</template>

<style scoped>
.schema-field {
  margin-bottom: 1rem;
}
</style>
