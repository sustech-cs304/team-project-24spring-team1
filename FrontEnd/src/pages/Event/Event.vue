<template>
  <div>
    <top-navbar class="fixed-top"></top-navbar>
    <div class="content"style="margin-top: 80px;">
      <fade-transition :duration="100" mode="out-in">
        <div class="col-lg-4" :class="{ 'text-right': false }">
          <card type="nav-tabs" class="text-left" style="width: 70rem; left: 11rem">
            <div v-if="error" class="error">{{ error }}</div>
            <div v-else>
<!--              <div slot="header" class="card-header-primary">-->
<!--                EventID: {{ getEventID() }}-->
<!--              </div>-->
              <br>
              <div class="d-flex">
                <div class="col">
                  <card class="card" style="width: 30rem;">
                    <img slot="image" class="card-img-top" :src="`events/${getEventID()}/1.jpg`" alt="Card image cap">
                    <p class="card-text">Picture 1</p>
                  </card>
                </div>
                <div class="col">
                  <div>
                    <h2 class="card-title">{{ event.name }}</h2>
                    <div>
                      <base-button class="animation-on-hover" simple type="primary">{{ event.kind }}</base-button>
                    </div>
                    <br>
                    <div>
                      <i class="tim-icons icon-time-alarm" style="display: inline-block;"></i>
                      <span style="margin-left: 10px;"></span>
                      <p class="card-text" style="display: inline-block;">{{ event.start_at }} - {{ event.end_at }}</p>
                    </div>

                    <div>
                      <i class="tim-icons icon-square-pin" style="display: inline-block;"></i>
                      <span style="margin-left: 10px;"></span>
                      <p class="card-text" style="display: inline-block;">{{ event.venue.name }}</p>
                    </div>

                    <div>
                      <i class="tim-icons icon-single-02" style="display: inline-block;"></i>
                      <span style="margin-left: 10px;"></span>
                      <p class="card-text" style="display: inline-block;">{{ event.organizer.name }}</p>
                    </div>
                    <br>
                    <div>
                      <h4 class="card-text" style="display: inline-block;">Tickets Left: {{event.tickets}}</h4>
                    </div>

                    <br>
                    <div>
                      <base-button class="animation-on-hover" type="primary" @click="registerForEvent">Register</base-button>
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
                    <h4 class="card-title">{{ comment.account.name }}</h4>
                    <p class="card-text">{{ comment.content }}</p>
                    <p class="card-text"><small class="text-muted">{{ comment.created_at }}</small></p>
                  </card>
                </div>
              </div>

            </div>
          </card>
        </div>
      </fade-transition>
    </div>
  </div>
</template>

<script>
import TopNavbar from "@/layout/dashboard/TopNavbar.vue";
import axios from "axios";
export default {
  components: {
    TopNavbar,
  },
  data() {
    return {
      eventId: null,
      event: {},
      error: null,
      comments: [],
      newComment: '', // 用于存储新评论的内容
    };
  },
  mounted() {
    this.eventId = this.$route.params.id;
    this.fetchEventData(this.eventId);
    this.getComments();
  },

  methods: {
    getEventID(){
      return this.eventId;
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
            alert('Successfully registered for the event.');
          })
          .catch(error => {
            console.error('Error Register:', error);
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
        }
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
