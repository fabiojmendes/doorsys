<script setup>
import { RouterLink } from 'vue-router'
</script>

<template>
  <nav class="navbar navbar-expand-sm bg-dark" data-bs-theme="dark">
    <div class="container main-container">
      <button
        class="navbar-toggler"
        type="button"
        data-bs-toggle="collapse"
        data-bs-target="#main-menu"
      >
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" id="main-menu">
        <div class="navbar-nav">
          <RouterLink class="navbar-brand" to="/">Doorsys</RouterLink>
          <RouterLink class="nav-item nav-link" to="/customers">Customers</RouterLink>
          <RouterLink class="nav-item nav-link" to="/logs">Logs</RouterLink>
        </div>
      </div>
    </div>
  </nav>
</template>
