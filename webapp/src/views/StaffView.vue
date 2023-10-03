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
  try {
    const res = await api.get(`/staff/${id}`)
    staff.value = res.data
  } catch (e) {
    console.log('error:')
  }
}

async function save() {
  const res = await api.put(`/staff/${staff.value.id}`, staff.value)
  staff.value = res.data
}

async function resetPin() {
  const confirmed = confirm(`Reset pin for ${staff.value.name}?`)
  console.log('Regenerate pin', confirmed)
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
          <input type="text" v-model="staff.fob" class="form-control" />
        </div>
        <div>
          <label for="pin" class="form-label">Pin</label>
        </div>
        <div class="input-group mb-3">
          <input type="text" v-model="staff.pin" class="form-control" readonly />
          <button type="button" class="btn btn-outline-danger" title="Reset Pin" @click="resetPin">
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
