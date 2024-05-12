import DashboardLayout from "@/layout/dashboard/DashboardLayout.vue";
import NotFound from "@/pages/NotFoundPage.vue";
import AdminLayout from "@/layout/admin/AdminLayout.vue";
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
const Login = () => import('@/pages/login');
const Event = () =>
    import('@/pages/Event/Event.vue');
const AdminForm = () =>
    import('@/adminform/components/dialogInd.vue');

const routes = [
  {
    path: '/login',
    component: Login,
    hidden:true,
  },
  {
    path: '/register',
    component: () => import('@/pages/register'),
    hidden: true
  },
  {
    path: '/',
    // component: () => import('@/pages/register'),
    redirect: '/login'// '/dashboard/dashboard',
  },
  {
    path: '/admin',
    name: 'admin',
    component: AdminLayout,
    children:[
      {
        path: 'publish',
        name: 'publish',
        component: AdminForm, //adminPublish, // 不用const可以吗
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
    ]
  },
  {
    path: "/dashboard",
    component: DashboardLayout,
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
      {
        path: "event",
        name: "event",
        component: Event,
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
