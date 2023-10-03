<script setup>
import { onMounted, ref } from 'vue'

const props = defineProps({ customer: Object })

const staff = ref([
  { id: 1, name: 'Staff 1', phone: '(111) 222 3344', pin: '1234' },
  { id: 2, name: 'Staff 2', phone: '(111) 222 3344', pin: '5678' },
  { id: 3, name: 'Staff 3', phone: '(111) 222 3344', pin: '9012', fob: '192038102938' }
])

const newStaff = ref({})

onMounted(async () => {})

async function addStaffMember() {
  console.log(newStaff.value)
  staff.value.push(newStaff.value)
}
</script>

<template>
  <div class="container">
    <h5>Staff Members</h5>
    <form class="row row-cols-lg-auto g-3" @submit.prevent="addStaffMember">
      <div class="col-4">
        <div class="input-group input-group-sm mb-3">
          <input
            v-model="newStaff.name"
            type="text"
            class="form-control form-control-sm"
            placeholder="Name"
            required
          />
        </div>
      </div>
      <div class="col-3">
        <div class="input-group input-group-sm mb-3">
          <input
            v-model="newStaff.phone"
            type="text"
            class="form-control form-control-sm"
            placeholder="Phone"
            required
          />
        </div>
      </div>
      <div class="col">
        <div class="input-group input-group-sm mb-3">
          <input
            v-model="newStaff.fob"
            type="text"
            class="form-control form-control-sm"
            placeholder="Fob"
          />
        </div>
      </div>
      <div class="col">
        <button type="submit" class="btn btn-primary btn-sm">Add</button>
      </div>
    </form>
    <ul class="list-group">
      <li
        v-for="s in staff"
        class="list-group-item d-flex justify-content-between align-items-center"
      >
        <span>
          {{ s.name }} <small class="text-body-secondary">{{ s.phone }}</small>
        </span>
        <span>
          <span v-if="s.fob" class="ms-1 badge bg-primary rounded-pill">fob</span>
          <span v-if="s.pin" class="ms-1 badge bg-primary rounded-pill">pin</span>
        </span>
      </li>
    </ul>
  </div>
</template>

<style>
.rounded-pill {
  cursor: pointer;
}
</style>
