import VueRouter from "vue-router";
import routes from "./routes";

// configure router
const createRouter= () => new VueRouter({ //Router
  routes, // short for routes: routes
  linkExactActiveClass: "active",
  // mode: 'history',
  scrollBehavior: (to) => {
    if (to.hash) {
      return { selector: to.hash };
    } else {
      return { x:0 , y: 0 };
    }
  },
});

const router =createRouter()
export default router;

export function resetRouter() {
  const newRouter = createRouter()
  router.matcher = newRouter.matcher // reset router
}

