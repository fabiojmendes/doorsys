<script setup>
import { inject, onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import BackButton from '../components/BackButton.vue'

const api = inject('api')
const route = useRoute()

const staff = ref({})

onMounted(loadStaff)

async function loadStaff() {
  const id = route.params.id
  const res = await api.get(`/staff/${id}`)
  staff.value = res.data
}

async function save() {
  // Set fob to null if empty
  staff.value.fob ||= null
  const res = await api.put(`/staff/${staff.value.id}`, staff.value)
  staff.value = res.data
}

async function resetPin() {
  const confirmed = confirm(`Reset pin for ${staff.value.name}?`)
  if (confirmed) {
    const res = await api.post(`/staff/${staff.value.id}/pin`)
    staff.value = res.data
  }
}
</script>

<template>
  <BackButton />
  <div class="card mb-3 mt-3">
    <div class="card-body">
      <div class="card-title">
        <h5 class="">Staff</h5>
      </div>
      <form class="card-text" @submit.prevent="save">
        <div class="mb-3">
          <label for="name" class="form-label">Name</label>
          <input v-model="staff.name" type="text" class="form-control" required />
        </div>
        <div class="mb-3">
          <label for="email" class="form-label">Phone</label>
          <input v-model="staff.phone" type="tel" class="form-control" required />
        </div>
        <div class="mb-3">
          <label for="fob" class="form-label">Fob</label>
          <input type="number" v-model="staff.fob" class="form-control" placeholder="(Optional)" />
        </div>
        <div>
          <label for="pin" class="form-label">Pin</label>
        </div>
        <div class="input-group mb-3">
          <input type="text" v-model="staff.pin" class="form-control text-secondary" readonly />
          <button type="button" class="btn btn-outline-primary" title="Reset Pin" @click="resetPin">
            <i class="bi bi-arrow-clockwise"></i>
          </button>
        </div>
        <div class="d-inline-flex gap-2">
          <input type="submit" class="btn btn-primary" value="Save" />
        </div>
      </form>
    </div>
  </div>
</template>
