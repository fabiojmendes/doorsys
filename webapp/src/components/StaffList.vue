<script setup>
import { inject, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const api = inject('api')
const props = defineProps({ customer: Object })

const staffList = ref([])
const formName = ref({})
const newStaff = ref({})

onMounted(async () => {
  const res = await api.get(`/customers/${props.customer.id}/staff`)
  staffList.value = res.data
})

async function addStaffMember() {
  console.log(newStaff.value)
  const res = await api.post('/staff', { customerId: props.customer.id, ...newStaff.value })
  staffList.value.push(res.data)
  newStaff.value = {}
  formName.value.focus()
}
</script>

<template>
  <div class="border rounded p-3">
    <h5>Staff Members</h5>
    <form @submit.prevent="addStaffMember">
      <div class="row g-3 mb-3">
        <div class="col input-group input-group-sm">
          <span class="input-group-text">
            <i class="bi bi-person"></i>
          </span>
          <input
            v-model="newStaff.name"
            ref="formName"
            type="text"
            class="form-control form-control-sm"
            placeholder="Name"
            required="true"
          />
        </div>
      </div>
      <div class="row g-3 mb-3">
        <div class="col input-group input-group-sm">
          <span class="input-group-text">
            <i class="bi bi-telephone"></i>
          </span>
          <input
            v-model="newStaff.phone"
            type="text"
            class="form-control form-control-sm"
            placeholder="Phone"
            required="true"
          />
        </div>
        <div class="col input-group input-group-sm">
          <span class="input-group-text">
            <i class="bi bi-tag" title="Staff has fob"></i>
          </span>
          <input
            v-model="newStaff.fob"
            type="text"
            class="form-control form-control-sm"
            placeholder="Fob (Optional)"
            pattern="\d{8}"
          />
        </div>
      </div>
      <div class="text-end">
        <input type="reset" class="btn btn-secondary btn-sm" value="Reset" />
        <input type="submit" class="btn btn-primary btn-sm ms-2" value="Add" />
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
        <tr @click="router.push(`/staff/${s.id}`)" v-for="s in staffList">
          <td>{{ s.name }}</td>
          <td class="text-secondary">{{ s.phone }}</td>
          <td class="text-end">
            <i v-if="s.fob" class="ms-2 bi bi-tag" title="Staff has fob"></i>
            <i v-if="s.pin" class="ms-2 bi bi-123" title="Staff has pin"></i>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
