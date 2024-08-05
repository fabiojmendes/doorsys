<script setup>
import { inject, computed, watch, onMounted, ref } from 'vue'

const LABELS = {
  pin: '123',
  fob: 'tag'
}

const api = inject('api')

const loading = ref(false)
const filter = ref({
  startDate: new Date().toLocaleDateString('en-CA'),
  endDate: new Date().toLocaleDateString('en-CA'),
  customerId: null,
  deviceId: null
})
const customers = ref([])
const devices = ref([])
const entries = ref([])

const entryMap = computed(() => {
  return entries.value.reduce((acc, rawEntry) => {
    const entry = {
      ...rawEntry,
      eventDate: new Date(rawEntry.eventDate),
      codeTypeLabel: LABELS[rawEntry.codeType]
    }
    const date = entry.eventDate.toLocaleDateString()
    if (!acc[date]) {
      acc[date] = []
    }
    acc[date].push(entry)
    return acc
  }, {})
})

onMounted(async () => {
  const res = await api.get('/customers', { params: { active: true } })
  customers.value = res.data

  const res2 = await api.get('/devices')
  devices.value = res2.data

  await load(filter.value)
})

async function load(params) {
  loading.value = true
  const startDate = new Date(params.startDate + ' 00:00:00')
  const endDate = new Date(params.endDate + ' 23:59:59.999')

  const res = await api.get('/entry_logs', {
    params: { ...params, startDate, endDate }
  })
  entries.value = res.data
  loading.value = false
}

watch(filter, load, { deep: true })
</script>

<template>
  <div class="border rounded p-3 mt-3">
    <form @submit.prevent="load">
      <div class="row g-3 mb-3">
        <div class="col input-group input-group-sm">
          <span class="input-group-text">Start</span>
          <input v-model.lazy="filter.startDate" type="date" class="form-control" />
        </div>
        <div class="col input-group input-group-sm">
          <span class="input-group-text">End</span>
          <input v-model.lazy="filter.endDate" type="date" class="form-control" />
        </div>
      </div>
      <div class="row g-3 mb-3">
        <div class="col input-group input-group-sm">
          <span class="input-group-text">Customer</span>
          <select v-model="filter.customerId" class="form-select">
            <option :value="null">All</option>
            <option v-for="c in customers" :value="c.id">
              {{ c.name }}
            </option>
          </select>
        </div>
      </div>
    </form>
  </div>

  <ul class="nav nav-tabs mt-2">
    <li class="nav-item">
      <a
        class="nav-link"
        :class="filter.deviceId == null ? 'active' : ''"
        href="#"
        @click="filter.deviceId = null"
        >All</a
      >
    </li>
    <li v-for="d in devices" class="nav-item">
      <a
        class="nav-link"
        :class="d.id == filter.deviceId ? 'active' : ''"
        href="#"
        @click="filter.deviceId = d.id"
        >{{ d.name }}</a
      >
    </li>
  </ul>

  <table class="table table-striped">
    <thead>
      <tr>
        <th>Customer</th>
        <th>Staff</th>
        <th>Time</th>
        <th class="text-center">Attributes</th>
      </tr>
    </thead>
    <tbody>
      <template v-for="(list, date) in entryMap" :key="date">
        <tr class="table-dark text-center">
          <td colspan="4">{{ date }}</td>
        </tr>
        <tr v-for="e in list">
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
          <td :title="e.eventDate.toLocaleString([], { hour12: false })">
            {{
              e.eventDate.toLocaleTimeString([], {
                hour12: false,
                hour: '2-digit',
                minute: '2-digit'
              })
            }}
          </td>
          <td class="text-center">
            <i :class="`bi bi-${e.codeTypeLabel}`" :title="`Entry using ${e.codeType}`"></i>
            <i v-if="e.success" title="Successful entry" class="ms-1 bi bi-check-square"></i>
            <i
              v-else
              title="Invalid attempt"
              class="ms-1 text-danger bi bi-exclamation-octagon"
            ></i>
          </td>
        </tr>
      </template>
    </tbody>
  </table>
  <div class="text-center">
    <span v-if="loading">Loading...</span>
    <span v-else-if="entries.length === 0">No Results found</span>
  </div>
</template>
span
