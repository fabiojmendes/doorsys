<script setup>
import { inject, onMounted, ref } from 'vue'

const LABELS = {
  pin: '123',
  fob: 'tag'
}

const api = inject('api')
const entries = ref([])

onMounted(async () => {
  const res = await api.get('/entry_logs')
  entries.value = res.data.map((item) => {
    return {
      ...item,
      created: new Date(item.created),
      codeTypeLabel: LABELS[item.codeType]
    }
  })
})
</script>

<template>
  <table class="table table-striped">
    <thead>
      <tr>
        <th>Name</th>
        <th>Date</th>
        <th>Attributes</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="e in entries">
        <td>{{ e.name }}</td>
        <td>{{ e.created.toLocaleString() }}</td>
        <td>
          <i
            v-if="e.success"
            :class="`bi bi-${e.codeTypeLabel}`"
            :title="`Entry using ${e.codeType}`"
          ></i>
          <i v-else class="text-danger bi bi-exclamation-octagon"></i>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped></style>
