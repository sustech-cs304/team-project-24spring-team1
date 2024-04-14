import VueRouter from "vue-router";
import routes from "./routes";
import Event from '@/pages/Event/Event.vue';

// configure router
const createRouter = () => new VueRouter({ //Router
  routes, // short for routes: routes
  linkExactActiveClass: "active",
  // mode: 'history',
  scrollBehavior: (to) => {
    if (to.hash) {
      return { selector: to.hash };
    } else {
      return { x: 0, y: 0 };
    }
  },
});

const router = createRouter();

// 添加新的路由配置
router.addRoutes([
  {
    path: '/event/:id',
    name: 'Event',
    component: Event // 将 Event.vue 组件与 /event/:id 路径相关联
  }
]);

export default router;

export function resetRouter() {
  const newRouter = createRouter();
  router.matcher = newRouter.matcher; // reset router
}
