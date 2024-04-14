<template>
  <div>
    <div class="row">
      <div class="col-lg-4" :class="{ 'text-right': isRTL }">
        <card type="nav-tabs" class="text-left" style="width: 60rem;">
          <div v-if="error" class="error">{{ error }}</div>
          <div v-else>
            <div slot="header" class="card-header-primary">
              EventID: {{ getEventID() }}
            </div>
            <div class="d-flex">
              <div class="col">
                <div>
                  <h2 class="card-title">Theme: {{ event.title }}</h2>
                  <div>
                    <base-button class="animation-on-hover" type="warning">Classification: {{ event.classification }}</base-button>
                    <base-button class="animation-on-hover" type="default">Time: {{ event.time }}</base-button>
                  </div>
                  <card type="nav-tabs">
                    <div slot="header" class="card-header-success">
                      Content
                    </div>
                    <blockquote class="blockquote mb-0">
                      <p>{{ event.content }}</p>
                      <footer class="blockquote-footer">Hold by <cite title="Source Title">{{ event.holder }}</cite></footer>
                    </blockquote>
                  </card>
                </div>
              </div>
              <div class="col">
                <card class="card" style="width: 20rem;">
                  <img slot="image" class="card-img-top" :src="`events/${getEventID()}/1.jpg`" alt="Card image cap">
                  <p class="card-text">Picture 1</p>
                </card>
              </div>
            </div>
            <div class="d-flex justify-content-between align-items-center">
                <base-button class="animation-on-hover" type="primary">Reserve</base-button>
                <base-button class="animation-on-hover" type="danger">Cancel</base-button>
                <base-button class="animation-on-hover" type="success">Comment</base-button>
            </div>
            <div class="dropdown-divider"></div>
            <div slot="footer" class="text-muted">
              SUSTech
            </div>
          </div>
          <card>
            <base-input class="has-success" value="Comment here..."></base-input>
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
        </card>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  // props: {
  //   eventId: Number,
  // },
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
    fetchEventData(eventId) {
      const url = `events/${eventId}/config.json`;
      fetch(url)
        .then(response => {
          if (!response.ok) {
            throw new Error(`Network response was not ok. Status: ${response.status} ${response.statusText}`);
          }
          return response.json();
        })
        .then(data => {
          this.event = data;
        })
        .catch(error => {
          this.error = `Error fetching event data: ${error.message}`;
        });
    },
  }
};
</script>

<style scoped>
.error {
  color: red;
}
</style>
