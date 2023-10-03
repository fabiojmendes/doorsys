<script setup>
import { computed, inject, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import StaffList from '@/components/StaffList.vue'

const api = inject('api')
const route = useRoute()
const router = useRouter()

const emptyCode = { codeType: '' }

const customer = ref({})
const codes = ref([])
const newCode = ref({ ...emptyCode })
const editing = ref(false)
const isNewCustomer = computed(() => customer.value.id === undefined)

onMounted(loadCustomer)

function toggleEdit() {
  editing.value = !editing.value
}

async function loadCustomer() {
  const id = route.params.id
  if (id === 'new') {
    editing.value = true
    return
  }

  try {
    const customer_res = await api.get(`/customers/${id}`)
    customer.value = customer_res.data
    const code_res = await api.get(`/customers/${id}/codes`)
    codes.value = code_res.data
  } catch (e) {
    console.log('error:')
  }
}

async function save() {
  const u = customer.value
  if (u.id) {
    const res = await api.put(`/customers/${u.id}`, u)
    console.log(res)
  } else {
    const res = await api.post('/customers', u)
    customer.value = res.data
    console.log(res)
  }
  editing.value = false
}

async function addNewCode() {
  const code = { customerId: customer.value.id, ...newCode.value }
  try {
    const res = await api.post('/codes', code)
    codes.value.push(res.data)
    newCode.value = { ...emptyCode }
  } catch (e) {
    console.log(e.response)
  }
}
</script>

<template>
  <div class="card mb-3 mt-3">
    <div class="card-body">
      <form v-if="editing" @submit.prevent="save">
        <div class="mb-3">
          <label for="name" class="form-label">Name</label>
          <input v-model="customer.name" type="text" class="form-control" required />
        </div>
        <div class="mb-3">
          <label for="email" class="form-label">Email address</label>
          <input v-model="customer.email" type="email" class="form-control" required />
        </div>
      </form>
      <div v-else class="mb-3">
        <h5 class="card-title">{{ customer.name }}</h5>
        <p class="card-text">
          {{ customer.email }}
        </p>
      </div>
      <p class="d-inline-flex gap-2">
        <button type="button" class="btn btn-secondary" @click="router.back()">Back</button>
        <template v-if="editing">
          <button type="submit" class="btn btn-primary" v-if="!isNewCustomer" @click="toggleEdit">
            Cancel
          </button>
          <button type="submit" class="btn btn-primary" @click="save">Save</button>
        </template>
        <template v-else>
          <button type="button" class="btn btn-primary" @click="toggleEdit">Edit</button>
        </template>
      </p>
    </div>
  </div>

  <StaffList v-if="!isNewCustomer" :customer="customer" />
</template>

<style scoped>
div.card {
  width: 22rem;
}
</style>
