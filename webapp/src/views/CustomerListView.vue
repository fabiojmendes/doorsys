<script setup>
import { inject, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const api = inject('api')
const customers = ref([])

onMounted(async () => {
  const res = await api.get('/customers')
  customers.value = res.data
})

function openCustomer(id) {
  router.push({ path: `/customers/${id}` })
}
</script>

<template>
  <div class="container pt-2 ps-2">
    <h4 class="text-center">Customers</h4>
    <div class="col text-end">
      <RouterLink class="btn btn-primary btn-sm" to="/customers/new">Add</RouterLink>
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
          <td>{{ c.name }}</td>
          <td>{{ c.email }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
