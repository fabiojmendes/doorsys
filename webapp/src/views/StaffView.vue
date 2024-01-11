<script setup>
import { inject, onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import BackButton from '../components/BackButton.vue'

const api = inject('api')
const route = useRoute()

const staff = ref({})
const status = ref({})

onMounted(loadStaff)

async function loadStaff() {
  const id = route.params.id
  const res = await api.get(`/staff/${id}`)
  staff.value = res.data
}

async function save() {
  // Set fob to null if empty
  status.value = {}
  staff.value.fob ||= null
  try {
    const res = await api.put(`/staff/${staff.value.id}`, staff.value)
    staff.value = res.data
    status.value = { class: 'alert-success', message: 'Saved with success' }
  } catch (e) {
    status.value = {
      class: 'alert-danger',
      message: e,
      context: e.response?.data?.msg || e.response?.data
    }
  }
}

async function resetPin() {
  const confirmed = confirm(`Reset pin for ${staff.value.name}?`)
  if (confirmed) {
    const res = await api.post(`/staff/${staff.value.id}/pin`)
    staff.value = res.data
  }
}

async function updateStatus() {
  const confirmed = confirm(
    `${staff.value.active ? 'Deactivate' : 'Activate'} ${staff.value.name}?`
  )
  if (confirmed) {
    const res = await api.put(`/staff/${staff.value.id}/status`, !staff.value.active)
    staff.value = res.data
  }
}
</script>

<template>
  <BackButton />
  <div v-if="!staff.active" class="alert alert-secondary mt-3" role="alert">
    This staff member is inactive
  </div>
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
          <input
            type="number"
            max="16777216"
            v-model="staff.fob"
            class="form-control"
            placeholder="(Optional)"
          />
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
          <button v-if="staff.active" type="button" class="btn btn-danger" @click="updateStatus">
            Deactivate
          </button>
          <button v-else type="button" class="btn btn-success" @click="updateStatus">
            Activate
          </button>
        </div>
        <div v-if="status.message" :class="status.class" class="mt-3 alert" role="alert">
          <p>{{ status.message }}</p>
          <template v-if="status.context">
            <hr />
            <p>{{ status.context }}</p>
          </template>
        </div>
      </form>
    </div>
  </div>
</template>
