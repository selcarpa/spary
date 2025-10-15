<script setup lang="ts">
import {ref, computed} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n';
import {groupRepository} from "@/entities/group.ts";
import {notify} from "@/components/notify/notifyStore.ts";

const { t } = useI18n()

const groupName = ref('')
const groupNameRule = [
  (value: string): boolean | string => {
    if (value?.length >= 1 && value?.length <= 15) return true
    return t('addGroup.groupNameRule')
  },
]

const groupSubscribeUrl = ref<string | null>(null)
const groupSubscribeUrlRule = [
  (value: string): boolean | string => {
    if (!value) {
      return true
    }
    try {
      new URL(value)
      return true
    } catch (e) {
      return t('addGroup.urlInvalid')
    }
  },
]

const groupArguments = ref<string | null>(null)
const isAdding = ref(false)

const isAddDisabled = computed(() => {
  return !groupName.value || groupName.value.length < 3 || groupName.value.length > 15 || isAdding.value
})

async function addGroup() {
  if (!isAddDisabled.value) {
    await add_group(groupName.value, groupSubscribeUrl.value, groupArguments.value)

    const check_repeat_one = await groupRepository.findByName(groupName.value)
    console.log(check_repeat_one)
    if (check_repeat_one.length > 0) {
      notify(t('addGroup.groupExists'), {
        color: "error"
      })
      return
    }

    await groupRepository.insert(
        {
          created_at: null, id: null, updated_at: null,
          name: groupName.value,
          url: groupSubscribeUrl.value,
          arguments: groupArguments.value
        }
    )
    notify(t('addGroup.groupAdded'))
  }
}

async function add_group(groupName: string, groupSubscribeUrl: string | null, groupArguments: string | null) {
  isAdding.value = true
  try {
    await invoke("add_group", {groupName, groupSubscribeUrl, groupArguments})
  } finally {
    isAdding.value = false
  }
}

</script>

<template>
  <v-container>
    <v-sheet class="mx-auto" width="80vw">
      <v-form fast-fail @submit.prevent>
        <v-text-field
            v-model="groupName"
            :rules="groupNameRule"
            :label="$t('addGroup.groupName')"
        ></v-text-field>

        <v-text-field
            v-model="groupSubscribeUrl"
            :rules="groupSubscribeUrlRule"
            :label="$t('addGroup.subscribeUrl')"
        ></v-text-field>
        <v-textarea
            v-model="groupArguments"
            :label="$t('addGroup.arguments')"
        ></v-textarea>
        <v-btn
            class="mt-2"
            type="submit"
            block
            :disabled="isAddDisabled"
            :loading="isAdding"
            @click="addGroup">
          {{ $t('addGroup.addGroupButton') }}
        </v-btn>
      </v-form>
    </v-sheet>
  </v-container>
</template>