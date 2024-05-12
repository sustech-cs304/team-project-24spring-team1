
import Vue from "vue";
import VueRouter from "vue-router";
import RouterPrefetch from "vue-router-prefetch";
import App from "./App";

import ElementUI from "element-ui";
import 'element-ui/lib/theme-chalk/index.css'
import '@/login/styles/index.scss' // global css
import '@/login/icons'

// TIP: change to import router from "./router/starterRouter"; to start with a clean layout
import router from "./router/index";
import BlackDashboard from "./plugins/blackDashboard";
import i18n from "./i18n";
import "./registerServiceWorker";

Vue.use(BlackDashboard);
Vue.use(VueRouter);
Vue.use(RouterPrefetch);
Vue.use(ElementUI);

Vue.config.productionTip = false
new Vue({
  router,
  i18n,
  render: (h) => h(App),
}).$mount("#app");

