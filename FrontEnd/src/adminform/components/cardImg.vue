<template>
  <el-row class="card-row" style="display: flex; justify-content: space-between; margin-top: 20px">
    <el-col :span="7">
      <el-card :body-style="{ padding: '0px' }">
        <img alt="image unable to render"
             src="@/adminform/assets/teaching_building.png"
             class="image"
        />
        <div style="padding: 2px">
          <span>teaching building</span>
          <div class="bottom">
            <time class="time">{{ currentDate }}</time>
            <el-button text class="button" @click="showMessage('teaching building')">view more</el-button>
          </div>
        </div>
      </el-card>
    </el-col>

    <el-col :span="7">
      <el-card :body-style="{ padding: '0px' }">
        <img alt="image unable to render"
             src="@/adminform/assets/domitory.png"
             class="image"
        />
        <div style="padding: 2px">
          <span>student domitory</span>
          <div class="bottom">
            <time class="time">{{ currentDate }}</time>
            <el-button text class="button" @click="showMessage('student domitory')">view more</el-button>
          </div>
        </div>
      </el-card>
    </el-col>
    <el-col :span="7">
      <el-card :body-style="{ padding: '0px' }">
        <img alt="image unable to render"
             src="@/adminform/assets/playground.png"
             class="image"
        />
        <div style="padding: 2px">
          <span>playground</span>
          <div class="bottom">
            <time class="time">{{ currentDate }}</time>
            <el-button text class="button" @click="showMessage('playground')">view more</el-button>
          </div>
        </div>
      </el-card>
    </el-col>
  </el-row>
</template>

<script>
export default {
  data() {
    return {
      currentDate: new Date().toISOString().split('T')[0],
    };
  },
  methods: {
    showMessage(location) {
      this.$alert(`This is ${location}`, 'Information', {
        confirmButtonText: 'OK',
      });
    },
  },
};
</script>

<style scoped>
.card-row {
  display: flex;
  justify-content: space-between;
}

.image {
  width: 100%;
  height: 180px; /* Set a fixed height to ensure equal height for all images */
  object-fit: cover; /* This ensures the image covers the area, maintaining aspect ratio */
}

.time {
  font-size: 12px;
  color: #999;
}

.bottom {
  margin-top: 13px;
  line-height: 12px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.button {
  padding: 0;
  min-height: auto;
}
</style>
