<script setup>
import { inject, computed, onMounted, ref } from 'vue'

const LABELS = {
  pin: '123',
  fob: 'tag'
}

const api = inject('api')
const entries = ref([])
const entryMap = computed(() => {
  return entries.value.reduce((acc, e) => {
    const date = e.eventDate.toLocaleDateString()
    if (!acc[date]) {
      acc[date] = []
    }
    acc[date].push(e)
    return acc
  }, {})
})

const customers = ref([])
const filter = ref({
  startDate: new Date().toLocaleDateString('en-CA'),
  endDate: new Date().toLocaleDateString('en-CA'),
  customerId: null
})

onMounted(async () => {
  const res = await api.get('/customers')
  customers.value = res.data

  load()
})

async function load() {
  const startDate = new Date(filter.value.startDate + ' 00:00:00')
  const endDate = new Date(filter.value.endDate + ' 23:59:59.999')

  const res = await api.get('/entry_logs', {
    params: {
      start_date: startDate,
      end_date: endDate,
      customer_id: filter.value.customerId
    }
  })
  entries.value = res.data.map((item) => {
    return {
      ...item,
      eventDate: new Date(item.eventDate),
      codeTypeLabel: LABELS[item.codeType]
    }
  })
}
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

      <div class="text-end">
        <input type="submit" class="btn btn-primary" value="Filter" />
      </div>
    </form>
  </div>

  <table class="table table-striped">
    <thead>
      <tr>
        <th>Door</th>
        <th>Customer</th>
        <th>Staff</th>
        <th>Time</th>
        <th class="text-center">Attributes</th>
      </tr>
    </thead>
    <tbody>
      <template v-for="(list, date) in entryMap" :key="date">
        <tr class="table-dark text-center">
          <td colspan="5">{{ date }}</td>
        </tr>
        <tr v-for="e in list">
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
  <div v-if="entries.length === 0" class="text-center">No Results found</div>
</template>
