<script setup lang="ts">
import {ref, onMounted, computed} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import Database from '@tauri-apps/plugin-sql'

const db = ref<any>(null)
const db_ready = ref(false)

onMounted(async () => {
  db.value = await Database.load('sqlite:spary.db')
  db_ready.value = true
})

const groupName = ref('')
const groupNameRule = [
  (value: string): boolean | string => {
    if (value?.length >= 3 && value?.length <= 15) return true
    return 'Group name must be between 3 and 15 characters.'
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
  return !groupName.value || groupName.value.length < 3 || groupName.value.length > 15 || isAdding.value || !db_ready.value
})

async function addGroup() {
  if (!isAddDisabled.value) {
    add_group(groupName.value, groupSubscribeUrl.value, groupArguments.value)

    const check_repeat_one = await db.value.select(
        "SELECT * FROM `group` WHERE name = ?",
        [groupName.value]
    )
    console.log(check_repeat_one)
    if (check_repeat_one.length > 0) {
      alert("Group already exists.")
      return
    }

    let add_result
    const params = [groupName.value];
    let sqlStmt = "INSERT INTO `group` (name";

    if (groupSubscribeUrl.value) {
      sqlStmt += ", url";
      params.push(groupSubscribeUrl.value);
    }

    if (groupArguments.value) {
      sqlStmt += ", arguments";
      params.push(groupArguments.value);
    }

    sqlStmt += ") VALUES (" + params.map(() => "?").join(", ") + ")";
    add_result = await db.value.execute(sqlStmt, params);

    if (add_result.rowsAffected > 0) {
      alert("Group added.")
    }
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