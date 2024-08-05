<script setup>
import { watch, inject, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const api = inject('api')
const customers = ref([])
const filter = ref({ active: true })

onMounted(async () => {
  await load(filter.value)
})

async function load(params) {
  const res = await api.get('/customers', { params })
  customers.value = res.data
}

function openCustomer(id) {
  router.push({ path: `/customers/${id}` })
}

watch(filter, load, { deep: true })
</script>

<template>
  <div class="container pt-2 ps-2">
    <h4 class="text-center">Customers</h4>
    <div class="text-end">
      <RouterLink class="btn btn-primary btn-sm" to="/customers/new">Add</RouterLink>
    </div>
    <div class="d-flex justify-content-end gap-2">
      <div class="form-check form-switch">
        <input class="form-check-input" type="checkbox" role="switch" v-model="filter.active" />
        <label class="form-check-label">Active</label>
      </div>
    </div>
    <table class="table table-hover">
      <thead>
        <tr>
          <th>Name</th>
          <th>Email</th>
        </tr>
      </thead>
      <tbody>
        <tr @click="openCustomer(c.id)" v-for="c in customers">
          <td :class="c.active ? '' : 'inactive'">{{ c.name }}</td>
          <td>{{ c.email }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
