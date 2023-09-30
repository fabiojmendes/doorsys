<script setup>
import { inject, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const api = inject('api')
const route = useRoute()
const router = useRouter()

const emptyCode = { codeType: '' }

const user = ref({})
const codes = ref([])
const newCode = ref({ ...emptyCode })
const editing = ref(false)

onMounted(async () => {
  const id = route.params.id
  if (id === 'new') {
    editing.value = true
    return
  }

  try {
    const user_res = await api.get(`/users/${id}`)
    user.value = user_res.data
    const code_res = await api.get(`/users/${id}/codes`)
    codes.value = code_res.data
  } catch (e) {
    console.log('error:')
  }
})

function toggleEdit() {
  editing.value = !editing.value
}

async function save() {
  const u = user.value
  if (u.id) {
    const res = await api.put(`/users/${u.id}`, u)
    console.log(res)
  } else {
    const res = await api.post('/users', u)
    user.value = res.data
    console.log(res)
  }
  editing.value = false
}

async function addNewCode() {
  const code = { userId: user.value.id, ...newCode.value }
  const res = await api.post('/codes', code)
  console.log(res)
  codes.value.push(res.data)
  newCode.value = { ...emptyCode }
}
</script>

<template>
  <div class="card mb-3 mt-3">
    <div class="card-body">
      <form @submit.prevent="save" v-if="editing">
        <div class="mb-3">
          <label for="name" class="form-label">Name</label>
          <input v-model="user.name" type="text" class="form-control" required />
        </div>
        <div class="mb-3">
          <label for="email" class="form-label">Email address</label>
          <input v-model="user.email" type="email" class="form-control" required />
        </div>
      </form>
      <div v-else class="mb-3">
        <h5 class="card-title">{{ user.name }}</h5>
        <p class="card-text">
          {{ user.email }}
        </p>
      </div>
      <p class="d-inline-flex gap-2">
        <button type="button" class="btn btn-secondary" @click="router.back()">Back</button>
        <template v-if="editing">
          <button type="submit" class="btn btn-primary" @click="toggleEdit">Cancel</button>
          <button type="submit" class="btn btn-primary" @click="save">Save</button>
        </template>
        <template v-else>
          <button type="button" class="btn btn-primary" @click="toggleEdit">Edit</button>
        </template>
      </p>
    </div>
  </div>

  <div class="card mb-3" v-if="user.id">
    <div class="card-body">
      <h5>Codes</h5>
      <form @submit.prevent="addNewCode">
        <div class="input-group input-group-sm mb-3">
          <select v-model="newCode.codeType" class="form-select form-select-sm" required>
            <option disabled value="">Type...</option>
            <option value="pin">Pin</option>
            <option value="fob">Fob</option>
          </select>
          <input v-model="newCode.code" type="text" class="form-control form-control-sm" required />
          <button class="btn btn-primary btn-sm" type="submit">Add</button>
        </div>
      </form>
      <ul class="list-group">
        <li
          v-for="c in codes"
          class="list-group-item d-flex justify-content-between align-items-center"
        >
          {{ c.code }}
          <span class="badge bg-primary rounded-pill">{{ c.codeType }}</span>
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
div.card {
  width: 22em;
}
</style>
