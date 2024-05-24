<template>
  <card type="user">
    <p class="card-text"></p>
    <div class="author">
      <div class="block block-one"></div>
      <div class="block block-two"></div>
      <div class="block block-three"></div>
      <div class="block block-four"></div>
      <a href="#">
        <img class="avatar" :src="imageUrl" alt="User Avatar" />
        <h5 class="title">Name: {{ user.name }}</h5>
        <h5 class="title">SID: {{ user.sid }}</h5>
      </a>
      <p class="description">
        Role: {{ user.role }}
      </p>
    </div>
    <p></p>
    <p class="card-description">
      {{ user.description }}
    </p>
    <div slot="footer" class="button-container">
      <base-button icon round class="primary">
        <i class="tim-icons icon-chat-33"></i>
      </base-button>
      <base-button icon round class="primary" @click="sendEmail">
        <i class="tim-icons icon-email-85"></i>
      </base-button>
    </div>
  </card>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      imageUrl: '',
      user: {
        id: null,
        sid: null,
        name: null,
        role: null,
        email: null,
        avatar: null,
        description: null
      },
      token: null
    };
  },
  mounted() {
    this.token = localStorage.getItem('token');
    if (!this.token) {
      console.log("Token not found.");
      return;
    }

    this.user.sid = localStorage.getItem('id');
    const apiUrl = `https://backend.sustech.me/api/account/${this.user.sid}/profile`;

    axios.get(apiUrl, {
      headers: {
        Authorization: `Bearer ${this.token}`
      }
    })

    .then(response => {
      const userData = response.data;
      this.user.sid = userData.sustech_id;
      this.user.name = userData.name;
      this.user.role = userData.role;
      this.user.email = userData.email;
      this.user.avatar = userData.avatar;
      this.user.description = userData.bio;
      this.imageUrl = `https://backend.sustech.me/uploads/${userData.avatar}.webp`;
    })
    .catch(error => {
      console.error('Error fetching profile data:', error);
    });
  },
  methods: {
    sendEmail() {
      const mailtoLink = `mailto:${this.user.email}`;
      window.location.href = mailtoLink;
    }
  }
}
</script>