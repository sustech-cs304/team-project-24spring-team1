<template>
  <card type="user">
    <div>
      <base-alert v-if="userNotFound" type="danger">
        <strong>Danger!</strong> User not found!
      </base-alert>
    </div>
    <p class="card-text"></p>
    <div class="author">
      <div class="block block-one"></div>
      <div class="block block-two"></div>
      <div class="block block-three"></div>
      <div class="block block-four"></div>
      <a href="#">
        <img class="avatar" :src="imageUrl" alt="User Avatar" />
        <h5 class="title">Name: {{ userCurrent.name }}</h5>
        <h5 class="title">SID: {{ userCurrent.sid }}</h5>
      </a>
      <p class="description">
        Role: {{ userCurrent.role }}
      </p>
    </div>
    <p></p>
    <p class="card-description">
      {{ userCurrent.description }}
    </p>
    <div slot="footer" class="button-container">
      <base-button icon round class="primary" @click="Chat">
        <i class="tim-icons icon-chat-33"></i>
      </base-button>
      <base-button icon round class="primary" @click="sendEmail">
        <i class="tim-icons icon-email-85"></i>
      </base-button>
    </div>
  </card>
</template>

<script>
/**
     * AI-generated-content
     * tool: ChatGPT
     * version: 3.5
     * usage: I used many prompts , and
     * directly copy the code from its response
     */
import axios from 'axios';

export default {
  data() {
    return {
      imageUrl: '',
      userCurrent: {
        id: null,
        sid: null,
        name: null,
        role: null,
        email: null,
        avatar: null,
        description: null
      },
      token: null,
      id: null,
      userNotFound: false
    };
  },
  mounted() {
    this.token = localStorage.getItem('token');
    this.id = localStorage.getItem('id');
    if (!this.token) {
      console.log("Token not found.");
      return;
    }

    const profileCurrentID = localStorage.getItem('profileCurrentID');
    if (!profileCurrentID) {
      this.userCurrent.id = this.id;
      this.userNotFound = true;
      console.log("User not found!");
    } else {
      console.log("Update userCurrent.id to", profileCurrentID);
      this.userCurrent.id = profileCurrentID;
    }

    const apiUrl = `https://backend.sustech.me/api/account/${this.userCurrent.id}/profile`;

    axios.get(apiUrl, {
      headers: {
        Authorization: `Bearer ${this.token}`
      }
    })
    .then(response => {
      const userData = response.data;
      this.userCurrent.sid = userData.sustech_id;
      this.userCurrent.name = userData.name;
      this.userCurrent.role = userData.role;
      this.userCurrent.email = userData.email;
      this.userCurrent.avatar = userData.avatar;
      this.userCurrent.description = userData.bio;
      this.imageUrl = `https://backend.sustech.me/uploads/${userData.avatar}.webp`;
    })
    .catch(error => {
      console.error('Error fetching profile data:', error);
    });
  },
  methods: {
    sendEmail() {
      const mailtoLink = `mailto:${this.userCurrent.email}`;
      window.location.href = mailtoLink;
    },
    Chat() {
      console.log("this.id is", this.id);
      console.log("this.userCurrent.id is", this.userCurrent.id);
      // Compare this.id and this.userCurrent.id
      if (this.id === this.userCurrent.id) {
        console.log("Cannot chat with yourself.");
        return;
      }

      // Send GET request to https://backend.sustech.me/api/chat
      axios.get('https://backend.sustech.me/api/chat/get_id', {
        params: {
          'with': this.userCurrent.id
        },
        headers: {
          Authorization: `Bearer ${this.token}`
        }
      })
      .then(response => {
        const chatId = response.data.chat_id;
        console.log('Chat ID:', chatId);

        // Send POST request to send a message
        const message = {
          content: `Hi! My sid is ${this.id}, and I wanna chat with you!`
        };
        
        axios.post(`https://backend.sustech.me/api/chat/${chatId}/message`, message, {
          headers: {
            Authorization: `Bearer ${this.token}`
          }
        })
        .then(() => {
          console.log('Message sent successfully');
          // Redirect to chat page
          this.$router.push('/dashboard/chat');
        })
        .catch(error => {
          console.error('Error sending message:', error);
        });

      })
      .catch(error => {
        console.error('Error fetching chat data:', error);
      });
    }
  }
}
</script>
