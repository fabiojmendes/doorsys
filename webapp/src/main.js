import 'bootstrap'
import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap-icons/font/bootstrap-icons.css'
import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'
import router from './router.js'
import axios from 'axios'

const app = createApp(App)

const api = axios.create({
  baseURL: '/api',
  headers: {
    'Content-Type': 'application/json'
  }
})

app.use(router).provide('api', api).mount('#app')
