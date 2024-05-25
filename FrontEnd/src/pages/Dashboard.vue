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
              <BaseCheckbox v-model="filter.checked" :id="'filter' + index" @change="applyFilter">
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
      <div class="col-12">
        <div class="row">
          <div v-for="(event, index) in events_all" :key="index" class="col-lg-4 mb-4" :class="{ 'text-right': false }">
            <card style="width: 23rem; margin-left: 10px">
  <!--            <img slot="image" class="card-img-top" :src="getEventImagePath(index)" :alt="event.title" style="width: 60rem; height: 16rem;" />-->
              <img class="card-img-top" :src="getEventImagePath(index)" alt="event.title" style="width: 60rem; height: 16rem;"/>
              <h4 class="card-title">{{ event.name }}</h4>
              <div>
                <i class="tim-icons icon-time-alarm" style="display: inline-block;"></i>
                <span style="margin-left: 10px;"></span>
                <p class="card-text" style="display: inline-block;">{{ event.start_at}}</p>
              </div>
              <div>
                <i class="tim-icons icon-square-pin" style="display: inline-block;"></i>
                <span style="margin-left: 10px;"></span>
                <p class="card-text" style="display: inline-block;">{{ event.venue.name }}</p>
              </div>
              <br>
              <base-button tag="a" simple type="primary" :href="getEventUrlPath(event.id)" role="button" aria-pressed="true"
                           class="animation-on-hover btn-center"> Event Detail
              </base-button>
            </card>
          </div>
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
      events: [],
      events_all: [],
      events_show: [],
      events_lecture: [],
      events_competition: [],
      filterParams: "",
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
      // return `events/${index + 1}/1.jpg`;
      return `https://backend.sustech.me/uploads/${this.events[index].cover}.webp`;
    },
    getEventUrlPath(index) {
      return `#/event/${index}`;
    },

    fetchEvents() {
      const url = 'https://backend.sustech.me/api/event';
      // axios.get(url)
      //     .then(response => {
      //       this.events_all = response.data.events;
      //     })
      //     .catch(error => {
      //       console.error('Error fetching events:', error);
      //       this.error = 'Error fetching events';
      //     });

      axios.get(url)
          .then(response => {
            this.events_all = response.data.events;
          })
          .catch(error => {
            console.error('Error fetching events:', error);
            this.error = 'Error fetching events';
          });

      axios.get(url, { params: { kind: "show" } })
          .then(response => {
            this.events_show = response.data.events;
          })
          .catch(error => {
            console.error('Error fetching events:', error);
            this.error = 'Error fetching events';
          });

      axios.get(url, { params: { kind: "lecture" } })
          .then(response => {
            this.events_lecture = response.data.events;
          })
          .catch(error => {
            console.error('Error fetching events:', error);
            this.error = 'Error fetching events';
          });

      axios.get(url, { params: { kind: "competition" } })
          .then(response => {
            this.events_competition = response.data.events;
          })
          .catch(error => {
            console.error('Error fetching events:', error);
            this.error = 'Error fetching events';
          });
    },

    applyFilter() {
      // Initialize an empty array to collect events based on filters
      let filteredEvents = [];

      // Check each filter and add the corresponding events to filteredEvents
      this.filters.forEach((filter, index) => {
        if (filter.checked) {
          if (filter.label === "show") {
            this.events = this.events_show;
          } else if (filter.label === "lecture") {
            this.events = this.events_lecture;
          } else if (filter.label === "competition") {
            this.events = this.events_competition;
          }
        }
      });

      // If no filters are selected, show all events
      if (filteredEvents.length === 0) {
        this.events = this.events_all;
      } else {
        // Remove duplicates and set the filtered events
        this.events = Array.from(new Set(filteredEvents));
      }
    }

  },
  mounted() {
    this.i18n = this.$i18n;
    if (this.enableRTL) {
      this.i18n.locale = "ar";
      this.$rtl.enableRTL();
    }
    this.fetchEvents();
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