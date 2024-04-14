<template>
  <div>
    <div class="row">
      <div class="col-lg-4" :class="{ 'text-right': isRTL }">

        <card style="width: 75rem; height: 30rem">
          <h1> {{ event.title }}</h1>
          <div v-if="error" class="error">{{ error }}</div>
          <div v-else>
            <p>{{ event.holder }}</p>
            <p>{{ event.time }}</p>
            <p>{{ event.classification }}</p>
            <p>{{ event.introduction }}</p>
          </div>
          <base-button type="info" @click="reserveActivity" style="padding: 10px 20px; font-size: 16px;  border-radius: 5px; cursor: pointer;">Reserve</base-button>
        </card>

      </div>
    </div>
  </div>
</template>

<script>
export default {
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
    }
  }
};
</script>

<style scoped>
.error {
  color: red;
}
</style>
