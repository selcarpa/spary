<script setup lang="ts">
import {ref} from 'vue'
import {useRouter} from "vue-router";
import { useI18n } from 'vue-i18n';
import {groupRepository} from "@/entities/group.ts";
import {nodeRepository} from "@/entities/node.ts";

const router = useRouter()
useI18n()

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

const panels = ref([])
const groups = ref<Group[]>([])

const loadData = async () => {

  const groupsInDb = await groupRepository.findAll()
  const nodesInDb = await nodeRepository.findAll()

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
}

function addItem(group: Group) {
  console.log(group.id)
  router.push(`/addNode/${group.id}`)
}

loadData()
</script>

<template>
  <v-container>
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
              {{ $t('nodeList.add') }}
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
                  <div>{{ $t('nodeList.usage', { traffic: item.traffic }) }}</div>
                </v-card-text>
              </v-card>
            </v-col>
          </v-row>
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>
  </v-container>
</template>

