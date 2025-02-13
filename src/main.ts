import { createApp } from "vue";
import App from "./App.vue";
import 'uno.css';

import naive from './naive'

const app = createApp(App);
app.use(naive)
app.mount("#app");

