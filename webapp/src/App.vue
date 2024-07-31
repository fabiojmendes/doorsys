<script setup>
import { onErrorCaptured, ref } from 'vue'
import { RouterView } from 'vue-router'
import Navbar from './components/Navbar.vue'
import { AxiosError } from 'axios'

const error = ref()

onErrorCaptured((err, vm, info) => {
  if (err instanceof AxiosError) {
    error.value = {
      message: err.message
    }
  } else {
    error.value = {
      message: err,
      context: info
    }
  }
  return false
})
</script>

<template>
  <Navbar />
  <section class="container main-container">
    <div v-if="error" class="mt-3 alert alert-danger" role="alert">
      <p>{{ error.message }}</p>
      <template v-if="error.context">
        <hr />
        <p>{{ error.context }}</p>
      </template>
    </div>
    <RouterView />
  </section>
</template>
