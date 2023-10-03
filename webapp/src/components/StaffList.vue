<script setup>
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const props = defineProps({ customer: Object })

const staff = ref([
  { id: 1, name: 'Staff 1', phone: '(111) 222 3344', pin: '1234' },
  { id: 2, name: 'Staff 2', phone: '(111) 222 3344', pin: '5678' },
  { id: 3, name: 'Staff 3', phone: '(111) 222 3344', pin: '9012', fob: '192038102938' }
])

const formName = ref({})
const newStaff = ref({})

onMounted(async () => {})

async function addStaffMember() {
  console.log(newStaff.value)
  staff.value.push(newStaff.value)
  newStaff.value = {}
  formName.value.focus()
}
</script>

<template>
  <div class="border rounded p-3">
    <h5>Staff Members</h5>
    <form class="row row-cols-lg-auto g-3" @submit.prevent="addStaffMember">
      <div class="col-12">
        <div class="input-group input-group-sm mb-3">
          <input
            v-model="newStaff.name"
            type="text"
            class="form-control form-control-sm"
            placeholder="Name"
            ref="formName"
            required
          />
        </div>
      </div>
      <div class="col-12">
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
      <div class="col-12">
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
        <div class="input-group input-group-sm mb-3">
          <input type="submit" class="btn btn-primary btn-sm" value="Add" />
        </div>
      </div>
    </form>
    <table class="table table-hover">
      <thead>
        <tr>
          <th>Name</th>
          <th>Phone</th>
          <th class="text-end">Fob/Pin</th>
        </tr>
      </thead>
      <tbody>
        <tr @click="router.push(`/staff/${s.id}`)" v-for="s in staff">
          <td>{{ s.name }}</td>
          <td>{{ s.phone }}</td>
          <td class="text-end">
            <i v-if="s.fob" class="ms-2 bi bi-tag" title="Staff has fob"></i>
            <i v-if="s.pin" class="ms-2 bi bi-123" title="Staff has pin"></i>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
