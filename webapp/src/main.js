import 'bootstrap'
import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap-icons/font/bootstrap-icons.css'
import 'vue-toastification/dist/index.css'
import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'
import router from './router.js'
import axios from 'axios'
import Toast, { POSITION } from 'vue-toastification'

const app = createApp(App)

const api = axios.create({
  baseURL: '/api',
  headers: {
    'Content-Type': 'application/json'
  }
})

const options = {
  position: POSITION.TOP_CENTER,
  timeout: 2000
}

app.use(router).use(Toast, options).provide('api', api).mount('#app')
