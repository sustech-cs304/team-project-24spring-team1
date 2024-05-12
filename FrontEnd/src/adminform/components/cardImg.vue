<template>
  <el-row class="card">
    <el-col
        v-for="(o, index) in 1"
        :key="o"
        :span="8"
        :offset="index > 0 ? 2 : 0"
    >
      <el-card :body-style="{ padding: '0px' }">
        <img alt="image unable to render"
            src="@/adminform/assets/bigRoom.png"
            class="image"
        />
        <div style="padding: 14px">
          <span>big conference room</span>
          <div class="bottom">
            <time class="time">{{ currentDate }}</time>
            <el-button text class="button">details</el-button>
          </div>
        </div>
      </el-card>
    </el-col>

    <el-col
        v-for="(o, index) in 1"
        :key="o"
        :span="8"
        :offset="index > 0 ? 2 : 0"
    >
      <el-card :body-style="{ padding: '0px' }">
        <img alt="image unable to render"
             src="@/adminform/assets/img.png"
             class="image"
        />
        <div style="padding: 14px">
          <span>small conference room</span>
          <div class="bottom">
            <time class="time">{{ currentDate }}</time>
            <el-button text class="button">details</el-button>
          </div>
        </div>
      </el-card>
    </el-col>

  </el-row>
</template>


<script lang="ts" setup>
import { ref } from 'vue'

const currentDate = ref(new Date())

</script>



<style>
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

.image {
  width: 100%;
  display: block;
}
</style>
