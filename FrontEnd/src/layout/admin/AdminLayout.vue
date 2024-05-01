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

export default {
  components: {
    BaseTable,
    TopNavbar,
    DashboardContent,
  },
  data() {
    return {
    };
  },
  methods: {
  },

};

</script>