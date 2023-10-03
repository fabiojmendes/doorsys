<script setup>
import { useRouter } from 'vue-router'
const router = useRouter()
</script>

<template>
  <a href="#" class="btn btn-sm btn-outline-dark mt-2" @click="router.back()">
    <i class="bi bi-chevron-left"></i> Back
  </a>
</template>
