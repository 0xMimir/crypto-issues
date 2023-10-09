import './assets/main.css'
import { createApp } from 'vue'
import { createPinia } from 'pinia'

import PrimeVue from 'primevue/config';
import App from './App.vue'
import router from './router'

import Column from 'primevue/column';
import DataTable from 'primevue/datatable';
import Paginator from 'primevue/paginator';

const app = createApp(App)
const pinia =  createPinia();

app.use(router)
app.use(pinia)
app.use(PrimeVue)
app.component("Column", Column)
app.component("DataTable", DataTable)
app.component("Paginator", Paginator)

app.mount('#app')
