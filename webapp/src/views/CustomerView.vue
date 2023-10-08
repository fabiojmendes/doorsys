<script setup>
import { computed, inject, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import StaffList from '@/components/StaffList.vue'
import BackButton from '@/components/BackButton.vue'

const api = inject('api')
const route = useRoute()
const router = useRouter()

const customer = ref({})
const editing = ref(false)
const isNewCustomer = computed(() => customer.value.id === undefined)

onMounted(load)

function toggleEdit() {
  editing.value = !editing.value
}

async function load() {
  const { id } = route.params
  if (id === 'new') {
    editing.value = true
    return
  }

  try {
    const res = await api.get(`/customers/${id}`)
    customer.value = res.data
  } catch (e) {
    console.log('error:')
  }
}

async function save() {
  const { id } = customer.value
  if (id) {
    const res = await api.put(`/customers/${id}`, customer.value)
    customer.value = res.data
  } else {
    const res = await api.post('/customers', customer.value)
    customer.value = res.data
    router.replace(`/customers/${customer.value.id}`)
  }
  editing.value = false
}
</script>

<template>
  <BackButton />
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
        <div class="mb-3">
          <label for="notes" class="form-label">Notes</label>
          <textarea v-model="customer.notes" class="form-control" />
        </div>
        <div class="d-inline-flex gap-2">
          <button type="button" class="btn btn-secondary" v-if="!isNewCustomer" @click="toggleEdit">
            Cancel
          </button>
          <input type="submit" class="btn btn-primary" value="Save" />
        </div>
      </form>

      <div v-else class="mb-3">
        <h5 class="card-title">
          {{ customer.name }}
        </h5>
        <p class="card-text">
          {{ customer.email }}
          {{ customer.notes }}
        </p>
        <button type="button" class="btn btn-primary" @click="toggleEdit">Edit</button>
      </div>
    </div>
  </div>

  <StaffList v-if="!isNewCustomer" :customer="customer" />
</template>
