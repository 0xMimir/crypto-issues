import './assets/main.css'
import "primevue/resources/themes/lara-light-teal/theme.css";
import "primevue/resources/primevue.min.css";

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import PrimeVue from 'primevue/config';
import App from './App.vue'
import router from './router'

import Column from 'primevue/column';
import DataTable from 'primevue/datatable';
import Paginator from 'primevue/paginator';
import TabMenu from 'primevue/tabmenu';
import Chart from 'primevue/chart';

const app = createApp(App)
const pinia =  createPinia();

app.use(router)
app.use(pinia)
app.use(PrimeVue)

app.component("Column", Column)
app.component("DataTable", DataTable)
app.component("Paginator", Paginator)
app.component("TabMenu", TabMenu)
app.component("Chart", Chart)

app.mount('#app')
