<script setup>
import { inject, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const api = inject('api')
const users = ref([])

onMounted(async () => {
  const res = await api.get('/users')
  users.value = res.data
})

function openUser(id) {
  router.push({ path: `/users/${id}` })
}
</script>

<template>
  <p class="pt-2 ps-2">
    <RouterLink class="btn btn-primary btn-sm" to="/users/new">Add</RouterLink>
  </p>
  <table class="table table-hover">
    <thead>
      <tr>
        <th>Name</th>
        <th>Email</th>
      </tr>
    </thead>
    <tbody>
      <tr @click="openUser(u.id)" v-for="u in users">
        <td>{{ u.name }}</td>
        <td>{{ u.email }}</td>
      </tr>
    </tbody>
  </table>
</template>

<style>
tbody tr {
  cursor: pointer;
}
</style>
