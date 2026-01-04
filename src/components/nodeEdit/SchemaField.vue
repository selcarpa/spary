<script setup lang="ts">
import {computed, toRaw, ref, watch} from 'vue';
import { z, ZodObject, ZodString, ZodNumber, ZodBoolean, ZodEnum, ZodOptional, ZodRecord, ZodArray, ZodIntersection, ZodLazy, ZodTuple, ZodMap, ZodSet, ZodFunction, ZodPromise, ZodAny, ZodUnknown, ZodVoid, ZodNull, ZodUndefined } from 'zod';

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

// For record type handling, maintain local state to avoid direct prop mutation issues
const recordData = ref<Record<string, any>>({});

// Watch for changes in props.modelValue and update local recordData
watch(() => props.modelValue, (newValue) => {
  recordData.value = newValue || {};
}, { immediate: true });

const typeName = computed(() => {
  if (rawSchema instanceof ZodObject) return 'ZodObject';
  if (rawSchema instanceof ZodString) return 'ZodString';
  if (rawSchema instanceof ZodNumber) return 'ZodNumber';
  if (rawSchema instanceof ZodBoolean) return 'ZodBoolean';
  if (rawSchema instanceof ZodEnum) return 'ZodEnum';
  if (rawSchema instanceof ZodOptional) return 'ZodOptional';
  if (rawSchema instanceof ZodRecord) return 'ZodRecord';
  // Check for union type using the internal typeName property
  if (rawSchema._def?.typeName === 'ZodUnion') return 'ZodUnion';
  // Additional check for union type that might not be caught by instanceof
  if (rawSchema._def?.options && Array.isArray(rawSchema._def.options)) return 'ZodUnion';
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

const recordValueSchema = computed(() => {
  if (rawSchema instanceof ZodRecord) {
    // Get the value schema from the record definition
    return rawSchema._def.valueType;
  }
  return null;
});

// Add debugging info for when record value schema is unknown
const recordValueType = computed(() => {
  if (recordValueSchema.value) {
    if (recordValueSchema.value instanceof ZodString) return 'ZodString';
    if (recordValueSchema.value instanceof ZodNumber) return 'ZodNumber';
    if (recordValueSchema.value instanceof ZodBoolean) return 'ZodBoolean';
    if (recordValueSchema.value instanceof ZodEnum) return 'ZodEnum';
    if (recordValueSchema.value instanceof ZodOptional) return 'ZodOptional';
    if (recordValueSchema.value instanceof ZodRecord) return 'ZodRecord';
    if (recordValueSchema.value instanceof ZodObject) return 'ZodObject';
    if (recordValueSchema.value._def && recordValueSchema.value._def.typeName === 'ZodUnion') return 'ZodUnion';
    // Additional check for union type in record values
    if (recordValueSchema.value._def?.options && Array.isArray(recordValueSchema.value._def.options)) return 'ZodUnion';
    if (recordValueSchema.value._def) return recordValueSchema.value._def.typeName;
    return 'UnknownValueType';
  }
  return 'NoValueSchema';
});

const updateRecordKey = (newKey: string, currentValue: any, oldKey: string) => {
  if (newKey === oldKey) return; // No change needed

  const newRecord = { ...recordData.value };
  delete newRecord[oldKey];
  newRecord[newKey] = currentValue;
  recordData.value = newRecord;
  emit('update:modelValue', newRecord);
};

const updateRecordValue = (key: string, newValue: any) => {
  const newRecord = { ...recordData.value };
  newRecord[key] = newValue;
  recordData.value = newRecord;
  emit('update:modelValue', newRecord);
};

const deleteRecordEntry = (key: string) => {
  const newRecord = { ...recordData.value };
  delete newRecord[key];
  recordData.value = newRecord;
  emit('update:modelValue', newRecord);
};

const addNewRecordEntry = () => {
  const newRecord = { ...recordData.value };
  // Generate a default key if none exists
  let newKey = 'newKey';
  let counter = 1;
  while (newRecord.hasOwnProperty(newKey)) {
    newKey = `newKey${counter}`;
    counter++;
  }
  // Determine a default value based on the schema type
  const defaultValue = getDefaultValueForSchema(recordValueSchema.value);
  newRecord[newKey] = defaultValue;
  recordData.value = newRecord;
  emit('update:modelValue', newRecord);
};

const getDefaultValueForSchema = (schema: z.ZodTypeAny) => {
  if (schema instanceof ZodString) return '';
  if (schema instanceof ZodNumber) return 0;
  if (schema instanceof ZodBoolean) return false;
  if (schema instanceof ZodArray) return [];
  if (schema._def && schema._def.typeName === 'ZodUnion') {
    // For union types, return the first option if possible
    if (schema._def.options && Array.isArray(schema._def.options)) {
      const firstOption = schema._def.options[0];
      if (firstOption instanceof ZodString) return '';
      if (firstOption instanceof ZodNumber) return 0;
      if (firstOption instanceof ZodBoolean) return false;
      if (firstOption instanceof ZodArray) return [];
      // For the first option, try to get its default value recursively
      return getDefaultValueForSchema(firstOption);
    }
    // If we can't determine, return a reasonable default
    return '';
  }
  if (schema instanceof ZodEnum) {
    // Return the first enum value as default
    if (schema._def.values && Array.isArray(schema._def.values) && schema._def.values.length > 0) {
      return schema._def.values[0];
    }
    // Fallback to first element in options if it's an array
    if (schema._def.options && Array.isArray(schema._def.options) && schema._def.options.length > 0) {
      return schema._def.options[0];
    }
    return '';
  }
  // For objects or other complex types, return appropriate defaults
  if (schema instanceof ZodObject) return {};
  return null;
};

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

    <template v-else-if="typeName === 'ZodRecord'">
      <v-card variant="outlined" class="pa-4 mb-4">
        <div class="text-subtitle-1 font-weight-medium">{{ label }}</div>
        <div v-for="[recordKey, recordValue] in Object.entries(recordData)" :key="recordKey" class="d-flex align-center mb-2">
          <v-text-field
              :model-value="recordKey"
              @update:modelValue="newKey => updateRecordKey(newKey, recordValue, recordKey)"
              label="Key"
              density="compact"
              class="mr-2"
          ></v-text-field>
          <SchemaField
              :schema="recordValueSchema"
              :model-value="recordValue"
              :label="'Value'"
              @update:modelValue="updateRecordValue(recordKey, $event)"
          />
          <v-btn icon="mdi-delete" size="small" @click="deleteRecordEntry(recordKey)" class="ml-2"></v-btn>
        </div>
        <v-btn @click="addNewRecordEntry" prepend-icon="mdi-plus" size="small">Add Entry</v-btn>
      </v-card>
    </template>

    <template v-else-if="typeName === 'ZodUnion'">
      <v-text-field
          v-model="value"
          :label="label"
          :hint="isOptional ? 'Optional' : ''"
          persistent-hint
      ></v-text-field>
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
