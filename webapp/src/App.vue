<script setup>
import { RouterView } from 'vue-router'
import Navbar from './components/Navbar.vue'
</script>

<template>
  <Navbar />
  <section class="container main-container">
    <RouterView />
  </section>
</template>
