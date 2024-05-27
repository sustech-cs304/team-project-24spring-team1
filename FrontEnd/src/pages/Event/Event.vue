<template>
  <div>
    <top-navbar class="fixed-top"></top-navbar>
    <div class="content"style="margin-top: 80px;">
      <fade-transition :duration="100" mode="out-in">
        <div class="col-lg-4" :class="{ 'text-right': false }">
          <card type="nav-tabs" class="text-left" style="width: 70rem; left: 11rem">
              <br>
              <div class="d-flex">
                <div class="col">
                  <card class="card" style="width: 30rem;">
                    <img slot="image" class="card-img-top" :src="`https://backend.sustech.me/uploads/${event.cover}.webp`" alt="Card image cap">
                    <p class="card-text">Picture 1</p>
                  </card>
                </div>
                <div class="col">
                  <div>
                    <h3 class="card-title">{{ event.name }}</h3>
                    <div>
                      <base-button class="animation-on-hover" simple type="primary">{{ event.kind }}</base-button>
                    </div>
                    <br>
                    <div>
                      <i class="tim-icons icon-time-alarm" style="display: inline-block;"></i>
                      <span style="margin-left: 10px;"></span>
                      <p class="card-text" style="display: inline-block;">{{ new Date(event.start_at).toLocaleString() }} - {{ new Date(event.end_at).toLocaleString() }}</p>
                    </div>

                    <div>
                      <i class="tim-icons icon-square-pin" style="display: inline-block;"></i>
                      <span style="margin-left: 10px;"></span>
                      <p class="card-text" style="display: inline-block;">{{ event.venue.name }}</p>
                    </div>

                    <div>
                      <i class="tim-icons icon-single-02" style="display: inline-block;"></i>
                      <span style="margin-left: 10px;"></span>
                      <a href="#/dashboard/profile" @click="handleClick(event.organizer.id)">{{ event.organizer.name }}</a>

                    </div>

                    <div>
                      <i class="tim-icons icon-bell-55" style="display: inline-block;"></i>
                      <span style="margin-left: 10px;"></span>
                      <p class="card-text" style="display: inline-block;">Register Deadline: {{ new Date(event.registration_deadline).toLocaleString() }}</p>
                    </div>
                    <br>
                    <div>
                      <h4 class="card-text" style="display: inline-block;">Total Tickets: {{event.tickets}}</h4>

                    </div>
                    <div>
                      <h4 class="card-text" style="display: inline-block;">Registered Number: {{event.participation_count}}</h4>
                    </div>


                    <br>
                    <div>
                      <base-button
                          v-if="!isRegister"
                          class="animation-on-hover"
                          type="primary"
                          @click="registerForEvent"
                      >
                        Register
                      </base-button>
                      <base-button
                          v-else
                          class="animation-on-hover"
                          type="danger"
                          @click="cancelRegisterEvent"
                      >
                        Cancel Registration
                      </base-button>
                    </div>

                    <div>
                      <button @click="share" class="btn btn-primary"> Share </button>
                    </div>

<!--                    <card type="nav-tabs">-->
<!--                      <div slot="header" class="card-header-success">-->
<!--                        Content-->
<!--                      </div>-->
<!--                      <blockquote class="blockquote mb-0">-->
<!--                        <p>{{ event.description }}</p>-->
<!--                        <footer class="blockquote-footer">Hold by <cite title="Source Title">{{ event.organizer.name }}</cite></footer>-->
<!--                      </blockquote>-->
<!--                    </card>-->
                  </div>
                </div>
              </div>

              <div class="dropdown-divider"></div>

              <br>
              <div class>
                <h2 class="card-title text-center">Event Description</h2>
                <p class>{{event.description}}</p>
              </div>

              <div class="dropdown-divider"></div>

              <br>
              <div class>
                <h2 class="card-title text-center">Comments</h2>
              </div>
              <card>
                <div style="display: flex; align-items: center;">
                  <input type="text" class="form-control" v-model="newComment" placeholder="Comment here..." style="color: black;">
                  <base-button class="animation-on-hover" type="success" style="margin-left: 10px;" @click="submitComment">Comment</base-button>
                </div>
              </card>


              <div>
                <div v-for="(comment, cIndex) in comments" :key="cIndex">
                  <card class="mb-3">
<!--                    <h4 class="card-title">{{ comment.account.name }}</h4>-->
                    <a href="#/dashboard/profile" @click="handleClick(comment.account.id)">{{ comment.account.name }}</a>
                    <p class="card-text">{{ comment.content }}</p>
                    <p class="card-text"><small class="text-muted">{{ new Date(comment.created_at).toLocaleString() }}</small></p>
                  </card>
                </div>
              </div>
          </card>
        </div>
      </fade-transition>
    </div>
  </div>
</template>

<script>
/**
     * AI-generated-content
     * tool: ChatGPT
     * version: 3.5
     * usage: I used many prompts , and
     * directly copy the code from its response
     */
import TopNavbar from "@/layout/dashboard/TopNavbar.vue";
import axios from "axios";
import BaseAlert from "@/components/BaseAlert.vue";
export default {
  components: {
    BaseAlert,
    TopNavbar,
  },
  data() {
    return {
      eventId: null,
      event: {},
      error: null,
      comments: [],
      newComment: '', // 用于存储新评论的内容
      errorMessage: '', // 用于存储错误信息
      isRegister: false, // 用于跟踪用户是否已注册
      participatedEvents: [],
    };
  },
  mounted() {
    this.eventId = this.$route.params.id;
    this.getRegisterStatus(this.eventId);
    this.fetchEventData(this.eventId);
    this.getComments();
  },

  methods: {
    getEventID(){
      return this.eventId;
    },

    share() {
      const commentUrl = `https://backend.sustech.me/api/moment`;
      const eventName = this.event.name;
      const eventOrganizer = this.event.organizer.name;
      const eventLocation = this.event.venue.name;
      // const eventStartTime = this.event.start_at;
      const eventStartTime = new Date(this.event.start_at).toLocaleDateString();

      const commentData = {
        content: `I've shared the event "${eventName}" organized by ${eventOrganizer}. The event will be held at ${eventLocation} starting on ${eventStartTime}. Come and join me!`
      };

      this.token = localStorage.getItem('token');
      if (!this.token) {
        console.log("Token not found.");
        return;
      }

      axios.post(commentUrl, commentData, {
        headers: {
          Authorization: `Bearer ${this.token}`
        }
      })
          .then(response => {
            // Handle successful post
            alert('Event Shared successfully.');

            // window.location.reload();
          })
          .catch(error => {
            console.error('Error posting moments:', error);
          });
    },

    getRegisterStatus(eventId){
      const url = `https://backend.sustech.me/api/event/${eventId}/participated`;
      this.token = localStorage.getItem('token');
      if (!this.token) {
        console.log("Token not found.");
        return;
      }
      axios.get(url, {
        headers: {
          Authorization: `Bearer ${this.token}`
        }
      })
          .then(response => {
            if (response.status === 204) {
              this.isRegister = true;
            }
          })
          .catch(error => {
            if (error.response && error.response.status === 404) {
              this.isRegister = false;
            } else {
              console.error('Error fetching participated events:', error);
            }
          });
    },


    fetchEventData(eventId){
      const url = `https://backend.sustech.me/api/event/${eventId}`;
      axios.get(url, {
        headers: {
        }
      })
          .then(response => {
            this.event = response.data;
          })
          .catch(error => {
            console.error('Error fetching event data:', error);
          });
    },

    handleClick(newID) {
      console.log('set profile ID =', newID)
      localStorage.setItem('profileCurrentID', newID);
      window.location.href = '#/dashboard/profile';
    },


    registerForEvent() {
      const apiUrl = `https://backend.sustech.me/api/event/${this.eventId}/register`;
      this.token = localStorage.getItem('token');
      if (!this.token) {
        console.log("Token not found.");
        return;
      }

      axios.post(apiUrl, {}, {
        headers: {
          Authorization: `Bearer ${this.token}`
        }
      })
          .then(response => {
            // Handle successful registration
            this.$message.success("Successfully registered for the event.");
            this.errorMessage = ''; // 清空错误消息
            this.isRegister = true; // 设置已注册状态
            window.location.reload();
          })
          .catch(error => {
            // 根据错误响应设置错误消息
            if (error.response) {
              switch (error.response.data.kind) {
                case 'record_not_found':
                  this.errorMessage = 'The event is not found.';
                  this.$message.error("The event is not found.")
                  break;
                case 'not_acceptable':
                  this.errorMessage = 'The registration deadline has passed.';
                  this.$message.error(error.response.data.message);
                  break;
                case 'record_already_exists':
                  this.errorMessage = 'You have already registered for the event.';
                  this.$message.error("You have already registered for the event.")
                  break;
                default:
                  this.errorMessage = 'An unknown error occurred.';
              }
            } else {
              this.errorMessage = 'Failed to register for the event.';
            }
            console.error('Error Register:', error);
          });
    },

    cancelRegisterEvent() {
      const apiUrl = `https://backend.sustech.me/api/event/${this.eventId}/register`;
      this.token = localStorage.getItem('token');
      if (!this.token) {
        console.log("Token not found.");
        return;
      }

      axios.delete(apiUrl, {
        headers: {
          Authorization: `Bearer ${this.token}`
        }
      })
          .then(response => {
            // 处理取消注册成功的情况
            this.$message.success("Successfully canceled registration for the event.");
            this.errorMessage = ''; // 清空错误消息
            this.isRegister = false; // 设置为未注册状态
            window.location.reload();
          })
          .catch(error => {
            // 根据错误响应设置错误消息
            if (error.response) {
              switch (error.response.data.kind) {
                case 'record_not_found':
                  this.errorMessage = 'The user has not registered for the event.';
                  this.$message.error("The user has not registered for the event.")
                  break;
                default:
                  this.errorMessage = 'An unknown error occurred.';
                  this.$message.error("An unknown error occurred.")
              }
            } else {
              this.errorMessage = 'Failed to cancel registration for the event.';
            }
            console.error('Error Cancel Register:', error);
          });
    },


    submitComment() {
      const commentUrl = `https://backend.sustech.me/api/event/${this.eventId}/comment`;
      const commentData = {
        content: this.newComment
      };

      this.token = localStorage.getItem('token');
      if (!this.token) {
        console.log("Token not found.");
        return;
      }

      axios.post(commentUrl, commentData, {
        headers: {
          Authorization: `Bearer ${this.token}`
        },
      })
          .then(response => {
            // Handle successful comment submission
            alert('Comment submitted successfully.');
            this.newComment = ''; // Clear the comment input
            this.getComments(); // Refresh the comments list
          })
          .catch(error => {
            console.error('Error submitting comment:', error);
          });
    },

    getComments() {
      const commentUrl = `https://backend.sustech.me/api/event/${this.eventId}/comment`;
      axios.get(commentUrl)
          .then(response => {
            this.comments = response.data.comments;
          })
          .catch(error => {
            console.error('Error fetching comments:', error);
          });
    },

  }
};
</script>

<style scoped>
.error {
  color: red;
}
.top-navbar {
  position: fixed;
  top: 0;
  width: 100%;
}

</style>
