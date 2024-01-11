<script setup>
import { inject, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const api = inject('api')
const props = defineProps({ customer: Object })

const staffList = ref([])
const formName = ref({})
const newStaff = ref({})
const status = ref({})

onMounted(async () => {
  const res = await api.get(`/customers/${props.customer.id}/staff`)
  staffList.value = res.data
})

async function addStaffMember() {
  status.value = {}
  try {
    const res = await api.post('/staff', { customerId: props.customer.id, ...newStaff.value })
    staffList.value.push(res.data)
    newStaff.value = {}
    formName.value.focus()
  } catch (e) {
    status.value = {
      message: e,
      context: e.response?.data?.msg
    }
  }
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
            type="number"
            max="16777216"
            class="form-control form-control-sm"
            placeholder="Fob (Optional)"
          />
        </div>
      </div>
      <div class="text-end">
        <input type="reset" class="btn btn-secondary btn-sm" value="Reset" />
        <input type="submit" class="btn btn-primary btn-sm ms-2" value="Add" />
      </div>
      <div v-show="status.message" class="mt-3 alert alert-danger" role="alert">
        <p>{{ status.message }}</p>
        <template v-if="status.context">
          <hr />
          <p class="mb-0">{{ status.context }}</p>
        </template>
      </div>
    </form>
    <table class="table table-hover">
      <thead>
        <tr>
          <th>Name</th>
          <th>Phone</th>
          <th class="text-end">Status</th>
        </tr>
      </thead>
      <tbody>
        <tr @click="router.push(`/staff/${s.id}`)" v-for="s in staffList">
          <td>
            <span v-if="s.active">{{ s.name }}</span>
            <s v-else>{{ s.name }}</s>
          </td>
          <td class="text-secondary">{{ s.phone }}</td>
          <td class="text-end">
            <i v-if="!s.active" class="ms-2 bi bi-ban" title="Staff is inactive"></i>
            <i v-if="s.fob" class="ms-2 bi bi-tag" title="Staff has fob"></i>
            <i v-if="s.pin" class="ms-2 bi bi-123" title="Staff has pin"></i>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
