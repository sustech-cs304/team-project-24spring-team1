<template>
  <div class="wrapper">
    <div class="main-panel">
      <top-navbar></top-navbar>
      <dashboard-content @click.native="toggleSidebar"> </dashboard-content>
    </div>
  </div>
</template>


<script>
import TopNavbar from "./TopNavbar.vue";
import DashboardContent from "./Content.vue";
import {BaseTable} from "@/components";
import 'element-ui/lib/theme-chalk/index.css';
import dialogInd from "@/adminform/components/dialogInd.vue";

export default {
  name: 'AdminLayout',
  components: {
    BaseTable,
    TopNavbar,
    DashboardContent,
  },
  data() {
    return {
      imgUrl:require("@/adminform/assets/th.jpg")
    };
  },
  methods: {
  },
};
</script>

<style>
.background{
  width:110%;
  height:120%;
  z-index:-1;
  position: absolute;
  opacity: 0.8;
}

.wrapper {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>