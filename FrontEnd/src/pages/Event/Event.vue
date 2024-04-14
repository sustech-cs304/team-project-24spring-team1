<template>
  <div>
    <h1>Event ID: {{ eventId }}</h1>
    <div v-if="error" class="error">{{ error }}</div>
    <div v-else>
      <h2>{{ event.holder }}</h2>
      <p>{{ event.time }}</p>
      <p>{{ event.classification }}</p>
      <p>{{ event.introduction }}</p>
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
