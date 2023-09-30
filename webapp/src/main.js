import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap'
import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import axios from 'axios'

const app = createApp(App)

const api = axios.create({
  baseURL: 'http://localhost:3000/',
  headers: {
    'Content-Type': 'application/json'
  }
})

app.use(router).provide('api', api).mount('#app')
