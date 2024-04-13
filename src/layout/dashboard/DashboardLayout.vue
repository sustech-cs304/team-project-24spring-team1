<template>
  <div class="wrapper">
    <side-bar>
      <template>
        <div class="filter-container-custom">
          <div
              class="filter-item"
              v-for="(filter, index) in filters"
              :key="index"
              :style="filterStyles(index)"
          >
            <BaseCheckbox v-model="filter.checked" :id="'filter' + index">
              {{ filter.label }}
            </BaseCheckbox>
          </div>
        </div>
      </template>
    </side-bar>
    <div class="main-panel">
      <top-navbar></top-navbar>
      <dashboard-content @click.native="toggleSidebar"> </dashboard-content>
      <content-footer></content-footer>
    </div>
  </div>
</template>

<style lang="scss">
.filter-container-custom { /* 添加样式 */
  display: flex;
  flex-direction: column;
}
.filter-item label {
  color: white; // 将字体颜色设置为白色
}

</style>

<script>
import TopNavbar from "./TopNavbar.vue";
import ContentFooter from "./ContentFooter.vue";
import DashboardContent from "./Content.vue";
import MobileMenu from "./MobileMenu";
import { BaseTable, BaseCheckbox } from "@/components"; // 导入 BaseCheckbox

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
      filters: [
        { label: "Lecture", checked: false },
        { label: "Concert", checked: false },
        { label: "Competition", checked: false },
        { label: "Social", checked: false },
        { label: "Volunteering", checked: false },
      ],
    };
  },
  methods: {
    toggleSidebar() {
      if (this.$sidebar.showSidebar) {
        this.$sidebar.displaySidebar(false);
      }
    },
    filterStyles(index) {
      return {
        marginTop: '10px',
        marginHeight: '10px',
        marginLeft: '20px', // 左边距
        marginBlock: '0px', // 垂直方向上边距
      };
    },
  },
};
</script>
