<script setup lang="ts">
import {ref, computed} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import {groupRepository} from "@/entities/group.ts";
import {notify} from "@/components/notify/notifyStore.ts";

const groupName = ref('')
const groupNameRule = [
  (value: string): boolean | string => {
    if (value?.length >= 1 && value?.length <= 15) return true
    return 'Group name must be between 1 and 15 characters.'
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
      return 'URL is not valid.'
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
      notify("Group already exists.", {
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
    notify("Group added.")
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
            label="Group name"
        ></v-text-field>

        <v-text-field
            v-model="groupSubscribeUrl"
            :rules="groupSubscribeUrlRule"
            label="Subscribe URL"
        ></v-text-field>
        <v-textarea
            v-model="groupArguments"
            label="Arguments"
        ></v-textarea>
        <v-btn
            class="mt-2"
            type="submit"
            block
            :disabled="isAddDisabled"
            :loading="isAdding"
            @click="addGroup">
          Add
        </v-btn>
      </v-form>
    </v-sheet>
  </v-container>
</template>