<script setup>
import { onErrorCaptured } from 'vue'
import { RouterView } from 'vue-router'
import Navbar from './components/Navbar.vue'
import { AxiosError } from 'axios'
import { useToast } from 'vue-toastification'

const toast = useToast()

onErrorCaptured((err, vm, info) => {
  if (err instanceof AxiosError) {
    const message = err.response?.data?.msg || err.response?.data || err.message
    toast.error(message)
    return false
  } else {
    toast.error('Oops! Unkown error')
  }
  return true
})
</script>

<template>
  <Navbar />
  <section class="container main-container">
    <RouterView />
  </section>
</template>
