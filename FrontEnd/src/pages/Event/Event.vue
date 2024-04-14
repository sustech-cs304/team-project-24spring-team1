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
      error: null
    };
  },
  mounted() {
    // 获取当前路由参数中的事件索引
    this.eventId = this.$route.params.id;
    // 根据事件索引动态加载事件信息
    this.fetchEventData(this.eventId);
  },
  methods: {
    fetchEventData(eventId) {
      // 发送请求加载事件信息
      fetch(`/event/${eventId}/config.json`)
        .then(response => {
          if (!response.ok) {
            throw new Error('Network response was not ok');
          }
          return response.json();
        })
        .then(data => {
          this.event = data;
        })
        .catch(error => {
          console.error('Error fetching event data:', error);
          this.error = 'Error fetching event data: ' + error.message;
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
