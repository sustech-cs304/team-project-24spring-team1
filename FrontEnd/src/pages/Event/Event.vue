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
                    <h2 class="card-title">Theme: {{ event.name }}</h2>
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
                      <i class="tim-icons icon-bank" style="display: inline-block;"></i>
                      <span style="margin-left: 10px;"></span>
                      <p class="card-text" style="display: inline-block;">{{ event.organizer.name }}</p>
                    </div>
                    <br>
                    <div>
                      <h4 class="card-text" style="display: inline-block;">Tickets Left: {{event.tickets}}</h4>
                    </div>

                    <br>
                    <div>
                      <base-button class="animation-on-hover" type="primary">Reserve</base-button>
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
                  <input type="text" class="form-control" placeholder="Comment here..." style="color: black;">
                  <base-button class="animation-on-hover" type="success" style="margin-left: 10px;">Comment</base-button>
                </div>
              </card>

              <div>
                <div v-for="(comment, cIndex) in event.comments" :key="cIndex">
                  <card class="mb-3">
                    <h4 class="card-title">{{ comment.username }}</h4>
                    <p class="card-text">{{ comment.comment }}</p>
                    <p class="card-text"><small class="text-muted">{{ comment.time }}</small></p>
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
    };
  },
  mounted() {
    this.eventId = this.$route.params.id;
    this.fetchEventData(this.eventId);
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
            console.error('Error fetching profile data:', error);
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
