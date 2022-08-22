import { createApp } from 'vue'
import App from './App.vue'
import Routes from './routes'
import './assets/icon/iconfont.js'
import './assets/icon/iconfont.css'
const app = createApp(App)
app.use(Routes)
app.mount('#app')
