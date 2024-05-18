<template>
  <div>
    <div class="row">
      <side-bar>
        <template>
          <div class="filter-container-custom">
            <div
                class="filter-item"
                v-for="(filter, index) in filters"
                :key="index"
                :style="filterStyles(index)"
            >
              <BaseCheckbox v-model="filter.checked" :id="'filter' + index">
                {{ filter.label }}
              </BaseCheckbox>
            </div>
          </div>
        </template>
      </side-bar>
    </div>
    <div class="row">
    </div>

    <div class="row">
      <div class="row">
        <div v-for="(event, index) in events" :key="index" class="col-lg-4 mb-4" :class="{ 'text-right': false }">
          <card style="width: 23rem; margin-left: 10px">
<!--            <img slot="image" class="card-img-top" :src="getEventImagePath(index)" :alt="event.title" style="width: 60rem; height: 16rem;" />-->
            <h4 class="card-title">{{ event.name }}</h4>
            <div>
              <i class="tim-icons icon-time-alarm" style="display: inline-block;"></i>
              <span style="margin-left: 10px;"></span>
              <p class="card-text" style="display: inline-block;">{{ event.start_at - event.end_at}}</p>
            </div>
            <div>
              <i class="tim-icons icon-square-pin" style="display: inline-block;"></i>
              <span style="margin-left: 10px;"></span>
              <p class="card-text" style="display: inline-block;">{{ event.location }}</p>
            </div>
            <br>
            <base-button tag="a" simple type="primary" :href="getEventUrlPath(index)" role="button" aria-pressed="true"
                         class="animation-on-hover btn-center"> Event Detail
            </base-button>
          </card>
        </div>
      </div>
    </div>


  </div>
</template>

<style lang="scss">
.filter-container-custom { /* 添加样式 */
  display: flex;
  flex-direction: column;
}
.filter-item label {
  color: white; // 将字体颜色设置为白色
}
</style>


<script>
import LineChart from "@/components/Charts/LineChart";
import BarChart from "@/components/Charts/BarChart";
import * as chartConfigs from "@/components/Charts/config";
import TaskList from "./Dashboard/TaskList";
import UserTable from "./Dashboard/UserTable";
import config from "@/config";
import {BaseCheckbox} from "@/components";
import Event from '@/pages/Event/Event.vue';
import axios from "axios";

export default {
  components: {
    LineChart,
    BarChart,
    TaskList,
    UserTable,
    BaseCheckbox,
    Event,
  },
  data() {
    return {
      filters: [
        {label: "show", checked: false},
        {label: "lecture", checked: false},
        {label: "competition", checked: false},
      ],
      // events: [
      //   {
      //     title: 'Card 1',
      //     description: 'Description for Card 1',
      //     link: '#'
      //   },
      //   {
      //     title: 'Card 2',
      //     description: 'Description for Card 2',
      //     link: '#'
      //   },
      //   {
      //     title: 'Card 3',
      //     description: 'Description for Card 3',
      //     link: '#'
      //   },
      //   {
      //     title: 'Card 4',
      //     description: 'Description for Card 4',
      //     link: '#'
      //   },
      //   {
      //     title: 'Card 5',
      //     description: 'Description for Card 5',
      //     link: '#'
      //   },
      //   {
      //     title: 'Card 6',
      //     description: 'Description for Card 6',
      //     link: '#'
      //   },
      // ],
      events: [],
    };
  },
  computed: {
    enableRTL() {
      return this.$route.query.enableRTL;
    },
    isRTL() {
      return this.$rtl.isRTL;
    },
  },
  methods: {
    filterStyles(index) { // 定义动态样式方法
      return {
        marginTop: '10px',
        marginHeight: '10px',
        marginLeft: '20px', // 左边距
        marginBlock: '0px', // 垂直方向上边距
      };
    },
    getEventImagePath(index) {
      return `events/${index + 1}/1.jpg`;
    },
    getEventUrlPath(index) {
      return `#/event/${index + 1}`;
      // return `#/dashboard/event/${index+1}`;
    },
  },
  mounted() {
    this.i18n = this.$i18n;
    if (this.enableRTL) {
      this.i18n.locale = "ar";
      this.$rtl.enableRTL();
    }
    const list_event_api = 'https://backend.sustech.me/api/event'
    axios.get(apiUrl, {
      headers: {
      }
    })
        .then(response => {
          const eventData = response.data;
          this.events = eventData.events;
        })
        .catch(error => {
          console.error('Error fetching profile data:', error);
        });
  },

  beforeDestroy() {
    if (this.$rtl.isRTL) {
      this.i18n.locale = "en";
      this.$rtl.disableRTL();
    }
  },
};
</script>
<style scoped>
.btn-center {
  display: block;
  margin: 0 auto;
}

</style>