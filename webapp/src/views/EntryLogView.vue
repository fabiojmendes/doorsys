<script setup>
import { inject, onMounted, ref } from 'vue'

const api = inject('api')

const entries = ref([])

onMounted(async () => {
  const res = await api.get('/entry_logs')
  entries.value = res.data.map((item) => {
    return {
      ...item,
      created: new Date(item.created)
    }
  })
})
</script>

<template>
  <table class="table table-striped">
    <thead>
      <tr>
        <th>Code</th>
        <th>Date</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="e in entries">
        <td>{{ e.code }}</td>
        <td>{{ e.created.toLocaleString() }}</td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped></style>
