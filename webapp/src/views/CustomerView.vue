<script setup>
import { computed, inject, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import StaffList from '@/components/StaffList.vue'
import BackButton from '@/components/BackButton.vue'

const api = inject('api')
const route = useRoute()
const router = useRouter()

const customer = ref({})
const staffList = ref([])
const editing = ref(false)
const isNewCustomer = computed(() => customer.value.id === undefined)

onMounted(load)
watch(customer, loadStaff)

function toggleEdit() {
  editing.value = !editing.value
}

async function load() {
  const { id } = route.params
  if (id === 'new') {
    editing.value = true
    return
  }

  const res = await api.get(`/customers/${id}`)
  customer.value = res.data
}

async function loadStaff(customer) {
  const staffRes = await api.get(`/customers/${customer.id}/staff`)
  staffList.value = staffRes.data
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

async function updateStatus() {
  const confirmed = confirm(
    `${customer.value.active ? 'Deactivate' : 'Activate'} ${customer.value.name}?`
  )
  if (confirmed) {
    const res = await api.put(`/customers/${customer.value.id}/status`, !customer.value.active)
    customer.value = res.data
  }
}
</script>

<template>
  <BackButton />
  <div v-if="customer.active === false" class="alert alert-secondary mt-3" role="alert">
    This customer is inactive
  </div>
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
        <div class="card-text">
          {{ customer.email }}
          <hr />
          <p>
            {{ customer.notes }}
          </p>
        </div>
        <div class="d-inline-flex gap-2">
          <button type="button" class="btn btn-primary" @click="toggleEdit">Edit</button>
          <button v-if="customer.active" type="button" class="btn btn-danger" @click="updateStatus">
            Deactivate
          </button>
          <button v-else type="button" class="btn btn-success" @click="updateStatus">
            Activate
          </button>
        </div>
      </div>
    </div>
  </div>

  <StaffList v-if="!isNewCustomer" :customer="customer" :staffList="staffList" />
</template>
