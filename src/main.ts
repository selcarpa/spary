/**
 * main.ts
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Plugins
import { registerPlugins } from '@/plugins'

// Components
import App from './App.vue'

// Composables
import { createApp } from 'vue'

// Styles
import 'unfonts.css'
import {createTerminal} from "vue-web-terminal";

const app = createApp(App)

registerPlugins(app)
app.use(createTerminal())

app.mount('#app')
