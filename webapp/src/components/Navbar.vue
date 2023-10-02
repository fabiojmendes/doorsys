<script setup>
import { RouterLink } from 'vue-router'
</script>

<template>
  <nav class="navbar navbar-expand-lg bg-dark" data-bs-theme="dark">
    <div class="container">
      <RouterLink class="navbar-brand" to="/">Doorsys</RouterLink>
      <button
        class="navbar-toggler"
        type="button"
        data-bs-toggle="collapse"
        data-bs-target="#main-menu"
      >
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" id="main-menu">
        <ul class="navbar-nav ms-auto">
          <li class="nav-item">
            <RouterLink class="nav-link" to="/customers">Customers</RouterLink>
          </li>
          <li class="nav-item">
            <RouterLink class="nav-link" to="/logs">Logs</RouterLink>
          </li>
        </ul>
      </div>
    </div>
  </nav>
</template>
