<template>
  <div class="wrapper">
    <div class="main-panel">
      <top-navbar></top-navbar>
      <dashboard-content @click.native="toggleSidebar"> </dashboard-content>
      <content-footer></content-footer>
    </div>
  </div>
</template>


<script>
import TopNavbar from "./TopNavbar.vue";
import ContentFooter from "./ContentFooter.vue";
import DashboardContent from "./Content.vue";
import MobileMenu from "./MobileMenu";
import {BaseTable, BaseCheckbox} from "@/components";

export default {
  components: {
    BaseTable,
    TopNavbar,
    ContentFooter,
    DashboardContent,
    BaseCheckbox,
  },
  data() {
    return {
    };
  },
  methods: {
  },

};

</script>