<script setup lang="ts">
import {ref, onMounted, computed} from 'vue'
import {invoke} from '@tauri-apps/api/core'
import Database from '@tauri-apps/plugin-sql'

defineProps<{
  groupId: string
}>()

const db = ref<any>(null)
const db_ready = ref(false)

onMounted(async () => {
  db.value = await Database.load('sqlite:spary.db')
  db_ready.value = true
})

const nodeAlias = ref('')

const nodeArguments = ref<string | null>(null)
const isAdding = ref(false)

const isAddDisabled = computed(() => {
  return !nodeAlias.value || nodeAlias.value.length < 3 || nodeAlias.value.length > 15 || isAdding.value || !db_ready.value
})

async function addNode() {
  if (!isAddDisabled.value) {
    await add_node(nodeAlias.value, nodeArguments.value)

    const check_repeat_one = await db.value.select(
        "SELECT * FROM `node` WHERE alias = ?",
        [nodeAlias.value]
    )
    console.log(check_repeat_one)
    if (check_repeat_one.length > 0) {
      alert("Node already exists.")
      return
    }

    let add_result
    const params = [nodeAlias.value];
    let sqlStmt = "INSERT INTO `node` (alias";

    if (nodeArguments.value) {
      sqlStmt += ", arguments";
      params.push(nodeArguments.value);
    }

    sqlStmt += ") VALUES (" + params.map(() => "?").join(", ") + ")";
    add_result = await db.value.execute(sqlStmt, params);

    if (add_result.rowsAffected > 0) {
      alert("Node added.")
    }
  }
}

async function add_node(nodeAlias: string, nodeArguments: string | null) {
  isAdding.value = true
  try {
    await invoke("add_node", {nodeAlias, nodeArguments})
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
            v-model="nodeAlias"
            label="Node alias"
        ></v-text-field>
        <v-select></v-select>

        <v-textarea
            v-model="nodeArguments"
            label="Arguments"
        ></v-textarea>
        <v-btn
            class="mt-2"
            type="submit"
            block
            :disabled="isAddDisabled"
            :loading="isAdding"
            @click="addNode">
          Add
        </v-btn>
      </v-form>
    </v-sheet>
  </v-container>
</template>