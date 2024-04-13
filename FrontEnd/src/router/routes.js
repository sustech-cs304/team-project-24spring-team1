import DashboardLayout from "@/layout/dashboard/DashboardLayout.vue";
// GeneralViews
import NotFound from "@/pages/NotFoundPage.vue";
// import { component } from "vue/types/umd.js";

// Admin pages
const Dashboard = () =>
  import(/* webpackChunkName: "dashboard" */ "@/pages/Dashboard.vue");
const Profile = () =>
  import(/* webpackChunkName: "common" */ "@/pages/Profile.vue");
const Notifications = () =>
  import(/* webpackChunkName: "common" */ "@/pages/Notifications.vue");
const Icons = () =>
  import(/* webpackChunkName: "common" */ "@/pages/Icons.vue");
const Maps = () => import(/* webpackChunkName: "common" */ "@/pages/Maps.vue");
const Typography = () =>
  import(/* webpackChunkName: "common" */ "@/pages/Typography.vue");
const TableList = () =>
  import(/* webpackChunkName: "common" */ "@/pages/TableList.vue");
const MyMoment = () =>
  import("@/pages/MyMoment.vue");
const MyEvent = () =>
  import("@/pages/MyEvent.vue");

const routes = [
  {
    path: "/",
    component: DashboardLayout,
    redirect: "/dashboard",
    children: [
      {
        path: "dashboard",
        name: "dashboard",
        component: Dashboard,
      },
      {
        path: "profile",
        name: "profile",
        component: Profile,
      },
      {
        path: "notifications",
        name: "notifications",
        component: Notifications,
      },
      {
        path: "icons",
        name: "icons",
        component: Icons,
      },
      {
        path: "maps",
        name: "maps",
        component: Maps,
      },
      {
        path: "typography",
        name: "typography",
        component: Typography,
      },
      {
        path: "table-list",
        name: "table-list",
        component: TableList,
      },
      {
        path: "mymoment",
        name: "mymoment",
        component: MyMoment,
      },
      {
        path: "myevent",
        name: "myevent",
        component: MyEvent,
      },
    ],
  },
  { path: "*", component: NotFound },
];

/**
 * Asynchronously load view (Webpack Lazy loading compatible)
 * The specified component must be inside the Views folder
 * @param  {string} name  the filename (basename) of the view to load.
function view(name) {
   var res= require('../components/Dashboard/Views/' + name + '.vue');
   return res;
};**/

export default routes;
