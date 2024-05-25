<style>
    .notification {
        position: absolute;
        top: 17px;
        right: 0px;
        width: 6px;
        height: 6px;
    }
</style>
<template>
  <nav
    class="navbar navbar-expand-lg navbar-absolute"
    :class="{ 'bg-white': showMenu, 'navbar-transparent': !showMenu }"
  >
    <div class="container-fluid">
      <div class="navbar-wrapper">
        <div
          class="navbar-toggle d-inline"
          :class="{ toggled: $sidebar.showSidebar }"

        >
          <button
            type="button"
            class="navbar-toggler"
            aria-label="Navbar toggle button"
            @click="toggleSidebar"
          >
            <span class="navbar-toggler-bar bar1"></span>
            <span class="navbar-toggler-bar bar2"></span>
            <span class="navbar-toggler-bar bar3"></span>
          </button>
        </div>
<!--        <a class="navbar-brand" href="#pablo">{{ routeName }}</a>-->
        <a class="navbar-brand" href="#/dashboard/dashboard">SUSTech Event</a>
      </div>
      <button
        class="navbar-toggler"
        type="button"
        @click="toggleMenu"
        data-toggle="collapse"
        data-target="#navigation"
        aria-controls="navigation-index"
        aria-label="Toggle navigation"
      >
        <span class="navbar-toggler-bar navbar-kebab"></span>
        <span class="navbar-toggler-bar navbar-kebab"></span>
        <span class="navbar-toggler-bar navbar-kebab"></span>
      </button>

      <collapse-transition>
        <div class="collapse navbar-collapse show" v-show="showMenu">
          <ul class="navbar-nav" :class="$rtl.isRTL ? 'mr-auto' : 'ml-auto'">
            <div>
              <base-button tag="a" round type="primary" href="#/admin/publish" role="button" aria-pressed="true">
                <i class="tim-icons el-icon-ice-cream-round"></i>PUBLISH</base-button>
            </div>
            <div>
              <base-button tag="a" round type="primary" href="#/dashboard/mymoment" role="button" aria-pressed="true">
                <i class="tim-icons icon-heart-2"></i>  My Moment</base-button>
            </div>
            <div>
              <base-button tag="a" round type="primary" href="#/dashboard/myevent" role="button" aria-pressed="true">
                <i class="tim-icons icon-bullet-list-67"></i>  My Event</base-button>
            </div>
            <div
              class="search-bar input-group"
              @click="searchModalVisible = true"
            >
              <input type="text" class="custom-input" placeholder="Search..." v-model="searchQuery">
              <!-- <div class="input-group-addon"><i class="tim-icons icon-zoom-split"></i></div> -->
              <!-- 在搜索按钮的点击事件上添加 search 方法 -->
              <button
                  class="btn btn-link"
                  id="search-button"
                  data-toggle="modal"
                  data-target="#searchModal"
                  @click="search"
              >
                <i class="tim-icons icon-zoom-split"></i>
              </button>
<!--               You can choose types of search input -->
            </div>
            <router-link to="/dashboard/chat">
              <base-button round icon type="primary">
                <i class="tim-icons icon-chat-33"></i>
              </base-button>
            </router-link>

            <base-dropdown
              tag="li"
              :menu-on-right="!$rtl.isRTL"
              title-tag="a"
              class="nav-item"
              menu-classes="dropdown-navbar"
            >
              <a
                slot="title"
                href="#"
                class="dropdown-toggle nav-link"
                data-toggle="dropdown"
                aria-expanded="true"
              >
                <div class="photo">
                  <img class="avatar" :src="imageUrl" alt="User Avatar" />
                </div>

                <b class="caret d-none d-lg-block d-xl-block"></b>
                <p class="d-lg-none">Log out</p>
              </a>
              <li class="nav-link">
                <a href="#/dashboard/profile" class="nav-item dropdown-item" @click="sendProfileMessage">Profile</a>
              </li>
              <li class="nav-link">
                <a href="#/dashboard/setting" class="nav-item dropdown-item" @click="sendProfileMessage">Settings</a>
              </li>
              <div class="dropdown-divider"></div>
              <li class="nav-link">
                <a href="#/login" class="nav-item dropdown-item">Log out</a>
              </li>
            </base-dropdown>
          </ul>
        </div>
      </collapse-transition>
    </div>
  </nav>
</template>

<script>
import { CollapseTransition } from "vue2-transitions";
import Modal from "@/components/Modal";
import axios from "axios";

export default {
  components: {
    CollapseTransition,
    Modal,
  },
  computed: {
    routeName() {
      return "SUSTech Event";
      // const { name } = this.$route;
      // return this.capitalizeFirstLetter(name);
    },
    isRTL() {
      return this.$rtl.isRTL;
    },
  },
  data() {
    return {
      activeNotifications: false,
      showMenu: false,
      searchModalVisible: false,
      searchQuery: "",
      imageUrl: localStorage.getItem('imageUrl'),
    };
  },

  methods: {

    search() {
      // 根据搜索关键字执行搜索操作
      // 这里可以使用axios或其他方法进行搜索
      // 假设搜索结果为events_show的一部分，如果搜索到结果，跳转到dashboard页面
      if (this.searchQuery.trim() !== "") {
        // this.$message.success("top-bar success");
          // 调用 Dashboard 组件的 receiveSearchResults 方法并传递搜索结果数据
          this.$root.$emit('search-results', this.searchQuery.trim());
        if (this.$route.path !== '/dashboard/dashboard') {
          this.$router.push('/dashboard/dashboard');
        }

      } else {
        // 如果搜索关键字为空，给出提示或其他操作
        alert("Please enter a search keyword.");
      }
    },

    capitalizeFirstLetter(string) {
      return string.charAt(0).toUpperCase() + string.slice(1);
    },
    toggleNotificationDropDown() {
      this.activeNotifications = !this.activeNotifications;
    },
    closeDropDown() {
      this.activeNotifications = false;
    },
    toggleSidebar() {
      this.$sidebar.displaySidebar(!this.$sidebar.showSidebar);
    },
    hideSidebar() {
      this.$sidebar.displaySidebar(false);
    },
    // toggleMenu() {
    //   this.showMenu = !this.showMenu;
    // },
    toggleMenu() {
      this.showMenu = !this.showMenu;
      // Navigate to Dashboard page
      this.$router.push('/dashboard/dashboard'); //'Dashboard'
    },
    sendProfileMessage() {
      console.log('from top bar to profile, set profileCurrentID =', localStorage.getItem('id'));
      localStorage.setItem('profileCurrentID', localStorage.getItem('id'));
    },
  },
};
</script>

<style>
.photo {
  text-align: center; /* 水平居中 */
}

.avatar {
  display: inline-block;
  max-width: 100%;    /* 防止图片超出容器边界 */
  max-height: 100%;
}
</style>