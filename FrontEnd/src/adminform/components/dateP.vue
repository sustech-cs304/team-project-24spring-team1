<template>
  <div>
    <el-form ref="params" :model="params" label-width="50px">
      <div>
        time
        <el-time-select
            style="width:100px;"
            :clearable="false"
            placeholder="起始时间"
            v-model="params.startTime"
            :picker-options="{
              start: '00:00',
              step: '00:01',
              end:params.endTime?params.endTime:'23:59',
              maxTime: params.endTime
          }">
        </el-time-select>
        <span class="">-</span>
        <el-time-select
            style="width:100px;"
            placeholder="结束时间"
            :clearable="false"
            v-model="params.endTime"
            :picker-options="{
              start: params.startTime?params.startTime:'00:00',
              step: '00:01',
              end:'24:00',
              minTime: params.startTime
            }">
        </el-time-select>
      </div>
    </el-form>
  </div>
</template>


<script>
export default {
  data() {
    return {
      params:{
        startTime: '08:00',
        endTime: '08:00',
      }
    }
  },
}
</script>


<style scoped>

</style>

