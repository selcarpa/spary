<script setup lang="ts">
import {ref, onMounted} from 'vue'
import Database from "@tauri-apps/plugin-sql";
import {useRouter} from "vue-router";
const router = useRouter()


interface GroupItem {
  name: string
  url: string
  traffic: string
}

interface Group {
  title: string
  items: GroupItem[]
  id: number
}

const db = ref<any>(null)
const panels = ref([])
const loading = ref(true) // 是否在加载中
const groups = ref<Group[]>([])

onMounted(async () => {
  db.value = await Database.load('sqlite:spary.db')

  const groupsInDb = await db.value.select(
      'SELECT * FROM `group`'
  )
  const nodesInDb = await db.value.select(
      'SELECT * FROM `node`'
  )

  for (let i = 0; i < groupsInDb.length; i++) {
    const group: any = groupsInDb[i]
    const nodes = nodesInDb.filter((node: any) => node.group_id === group.id)
    groups.value.push({
      title: group.name,
      items: nodes.map((node: any) => ({
        name: node.name,
        url: node.url,
        traffic: "0MB"
      })),
      id: group.id
    })
  }


  // 模拟异步加载（例如从 API 获取）
  await new Promise(resolve => setTimeout(resolve, 100))

  loading.value = false
})

function addItem(group: Group) {
  console.log(group.id)
  router.push(`/addNode/${group.id}`)
}
</script>

<template>
  <v-container>
    <!-- 加载中时显示骨架屏 -->
    <template v-if="loading">
      <v-skeleton-loader type="heading" class="mb-4"/>

      <v-row dense>
        <v-col
            v-for="i in 8"
            :key="i"
            cols="12"
            sm="6"
            md="4"
            lg="3"
        >
          <v-skeleton-loader type="card" class="ma-2"/>
        </v-col>
      </v-row>
    </template>

    <!-- 加载完成后显示实际内容 -->
    <template v-else>
      <v-expansion-panels v-model="panels" multiple>
        <v-expansion-panel
            v-for="(group, i) in groups"
            :key="i"
            elevation="2"
            class="my-3"
        >
          <v-expansion-panel-title>
            <div class="d-flex align-center justify-space-between w-100">
              <span class="text-h6">{{ group.title }}</span>

              <v-btn
                  size="small"
                  color="primary"
                  variant="tonal"
                  prepend-icon="mdi-plus"
                  @click.stop="addItem(group)"
              >
                add
              </v-btn>
            </div>
          </v-expansion-panel-title>

          <v-expansion-panel-text>
            <v-row dense>
              <v-col
                  v-for="(item, j) in group.items"
                  :key="j"
                  cols="12"
                  sm="6"
                  md="4"
                  lg="3"
              >
                <v-card elevation="3" class="ma-2">
                  <v-card-title class="text-subtitle-1">
                    {{ item.name }}
                  </v-card-title>
                  <v-card-text>
                    <div>{{ item.url }}</div>
                    <div>usage: {{ item.traffic }}</div>
                  </v-card-text>
                </v-card>
              </v-col>
            </v-row>
          </v-expansion-panel-text>
        </v-expansion-panel>
      </v-expansion-panels>
    </template>
  </v-container>
</template>

