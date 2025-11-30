<script setup lang="ts">
import {ref, onMounted, computed} from 'vue'
import { useI18n } from 'vue-i18n';
import {Group, groupRepository} from "@/entities/group.ts";
import {nodeRepository} from "@/entities/node.ts";
import {notify} from "@/components/notify/notifyStore.ts";
import {ConfigurationSchema} from "@/utils/core/configurator/schema/schema.ts";
import {CoreTypes} from "@/utils/core/CoreDef.ts";

const { t } = useI18n()

const props = defineProps<{ groupId: string }>()

const allGroups = ref<Group[]>([])
const selectedGroupId = ref<number | null>(null)
const configurationSchema= ref<ConfigurationSchema | null>(null)

const loadGroups = async () => {
  allGroups.value = await groupRepository.findAll()
  const group = allGroups.value.find(g => g.id === Number(props.groupId))
  selectedGroupId.value = group ? group.id : null
}

onMounted(loadGroups)

const nodeAlias = ref('')
const nodeArguments = ref<string | null>(null)
const isAdding = ref(false)

const isAddDisabled = computed(() => {
  return !nodeAlias.value || nodeAlias.value.length < 3 || nodeAlias.value.length > 15 || isAdding.value
})

async function addNode() {
  if (!isAddDisabled.value && selectedGroupId.value !== null) {
    isAdding.value = true
    await nodeRepository.insert({
      created_at: null,
      id: null,
      updated_at: null,
      alias: nodeAlias.value,
      arguments: nodeArguments.value,
      group_id: selectedGroupId.value
    })
    notify(t('addNode.nodeAddedSuccess'))
    isAdding.value = false
  }
}
</script>
<template>
  <v-container>
    <v-sheet class="mx-auto" width="80vw">
      <v-form fast-fail @submit.prevent>
        <v-select
            v-model="selectedGroupId"
            :label="$t('addNode.group')"
            :items="allGroups"
            item-title="name"
            item-value="id"
            variant="solo"
        ></v-select>

        <v-text-field
            v-model="nodeAlias"
            :label="$t('addNode.nodeAlias')"
        ></v-text-field>

        <v-select
            v-model="configurationSchema"
            :label="$t('addNode.nodeType')"
            :items="CoreTypes"
            item-title="name"
            variant="solo">
        </v-select>

        <v-btn
            class="mt-2"
            type="submit"
            block
            :disabled="isAddDisabled"
            :loading="isAdding"
            @click="addNode">
          {{ $t('addNode.addNodeButton') }}
        </v-btn>
      </v-form>
    </v-sheet>
  </v-container>
</template>
