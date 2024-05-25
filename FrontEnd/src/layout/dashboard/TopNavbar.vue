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
              <button
                  class="btn btn-link"
                  id="search-button"
                  data-toggle="modal"
                  data-target="#searchModal"
                  @click="search"
              >
                <i class="tim-icons icon-zoom-split"></i>
              </button>
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
      imageUrl: '',
    };
  },
  mounted() {
    this.fetchUserProfile();
  },
  methods: {
    fetchUserProfile() {
      const userId = localStorage.getItem('id');
      axios.get(`https://backend.sustech.me/api/account/${userId}/profile`)
        .then(response => {
          const avatar = response.data.avatar;
          this.imageUrl = `https://backend.sustech.me/uploads/${avatar}.webp`;
          localStorage.setItem('imageUrl', this.imageUrl);
        })
        .catch(error => {
          console.error('Error fetching user profile:', error);
        });
    },
    search() {
      if (this.$route.path !== '/dashboard/dashboard') {
        this.$router.push('/dashboard/dashboard');
      }
      this.$root.$emit('search-results', this.searchQuery.trim());
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
    toggleMenu() {
      this.showMenu = !this.showMenu;
      this.$router.push('/dashboard/dashboard');
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
  text-align: center;
}

.avatar {
  display: inline-block;
  max-width: 100%;
  max-height: 100%;
}
</style>
