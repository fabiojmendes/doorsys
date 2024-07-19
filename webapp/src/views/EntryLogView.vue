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
      eventDate: new Date(item.eventDate),
      codeTypeLabel: LABELS[item.codeType]
    }
  })
})
</script>

<template>
  <table class="table table-striped">
    <thead>
      <tr>
        <th>Door</th>
        <th>Customer</th>
        <th>Staff</th>
        <th>Date</th>
        <th class="text-center">Attributes</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="e in entries">
        <td>{{ e.deviceName }}</td>
        <td>
          <RouterLink v-if="e.customerId" :to="`/customers/${e.customerId}`">
            {{ e.customerName }}
          </RouterLink>
          <span v-else>None</span>
        </td>
        <td>
          <RouterLink v-if="e.staffId" :to="`/staff/${e.staffId}`">{{ e.staffName }}</RouterLink>
          <span v-else>{{ e.code }}</span>
        </td>
        <td>{{ e.eventDate.toLocaleString() }}</td>
        <td class="text-center">
          <i :class="`bi bi-${e.codeTypeLabel}`" :title="`Entry using ${e.codeType}`"></i>
          <i v-if="e.success" title="Successful entry" class="ms-1 bi bi-check-square"></i>
          <i v-else title="Invalid attempt" class="ms-1 text-danger bi bi-exclamation-octagon"></i>
        </td>
      </tr>
    </tbody>
  </table>
</template>
