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
                @change = "applyFilter"
            >
              <BaseCheckbox v-model="filter.checked">
                {{ filter.label }}
              </BaseCheckbox>

            </div>
          </div>

        </template>
      </side-bar>
    </div>
    <div class="row"></div>
    <div class="row">
      <div class="col-12">
        <div class="row">
          <div v-for="(event, index) in filter_events" :key="index" class="col-lg-4 mb-4" :class="{ 'text-right': false }">
            <card style="width: 23rem; margin-left: 10px">
              <img class="card-img-top" :src="getEventImagePath(index)" style="width: 60rem; height: 16rem;" />
              <h4 class="card-title">{{ index }}</h4>
              <h4 class="card-title">{{ getEventImagePath(index) }}</h4>
<!--              <h4 class="card-title">{{ event.name }}</h4>-->
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
.filter-container-custom {
  display: flex;
  flex-direction: column;
}
.filter-item label {
  color: white;
}
</style>

<script>
import { BaseCheckbox } from "@/components";
import axios from "axios";

export default {
  components: {
    BaseCheckbox,
  },
  data() {
    return {
      filters: [
        { label: "show", checked: false },
        { label: "lecture", checked: false },
        { label: "competition", checked: false },
      ],
      events: [],
      filter_events: [],
      events_show: [],
      events_lecture: [],
      events_competition: [],
      keyword: "",
    };
  },
  methods: {
    filterStyles(index) {
      return {
        marginTop: '10px',
        marginHeight: '10px',
        marginLeft: '20px',
        marginBlock: '0px',
      };
    },
    getEventImagePath(index) {
      // this.$message(this.events[index].cover);
      // console.log('aaa', this.filter_events);
      return `https://backend.sustech.me/uploads/${this.filter_events[index].cover}.webp`;
    },
    getEventUrlPath(id) {
      return `#/event/${id}`;
    },
    getFilterStatus() {
      const storedFilters = localStorage.getItem("filters");
      if (storedFilters) {
        this.filters = JSON.parse(storedFilters);
      }
      this.updateFilterEvents();
    },


    updateFilterEvents() {
      let filteredEvents = [];

      if (this.filters[0].checked) {
        filteredEvents = filteredEvents.concat(this.events_show);
      }
      if (this.filters[1].checked) {
        filteredEvents = filteredEvents.concat(this.events_lecture);
      }
      if (this.filters[2].checked) {
        filteredEvents = filteredEvents.concat(this.events_competition);
      }

      // filteredEvents = filteredEvents.concat(filtered_keyword);
      if (this.keyword !== "") {
        if(!this.filters[0].checked && !this.filters[1].checked && !this.filters[2].checked){
          filteredEvents = this.events.filter(event => {
            return event.name.toLowerCase().includes(this.keyword.toLowerCase());
          });
        }else{
          filteredEvents = filteredEvents.filter(event => {
            return event.name.toLowerCase().includes(this.keyword.toLowerCase());
          });
        }
      }
      if (this.keyword==="" && !this.filters[0].checked && !this.filters[1].checked && !this.filters[2].checked) {
        this.filter_events = this.events;
      } else {
        this.filter_events = filteredEvents;
      }
    },
    fetchEvents() {
      const url = 'https://backend.sustech.me/api/event';
      localStorage.setItem("filters",JSON.stringify(this.filters));

      axios.get(url)
          .then(response => {
            this.events = response.data.events;
            this.updateFilterEvents();
          })
          .catch(error => {
            console.error('Error fetching events:', error);
          });

      axios.get(url, { params: { kind: "show" } })
          .then(response => {
            this.events_show = response.data.events;
            this.updateFilterEvents();
          })
          .catch(error => {
            console.error('Error fetching events:', error);
          });

      axios.get(url, { params: { kind: "lecture" } })
          .then(response => {
            this.events_lecture = response.data.events;
            this.updateFilterEvents();
          })
          .catch(error => {
            console.error('Error fetching events:', error);
          });

      axios.get(url, { params: { kind: "competition" } })
          .then(response => {
            this.events_competition = response.data.events;
            this.updateFilterEvents();
          })
          .catch(error => {
            console.error('Error fetching events:', error);
          });
    },



    applyFilter() {
      // this.$message.success("haha");
      localStorage.setItem("filters", JSON.stringify(this.filters));
      this.updateFilterEvents();
    },

    receiveSearchParam(keyword) {
      this.keyword = keyword;
      // this.$message.success(this.keyword);
      this.updateFilterEvents();
    }
  },

  mounted() {
    // this.getFilterStatus();
    this.fetchEvents();
    this.updateFilterEvents();
    this.$root.$on('search-results', this.receiveSearchParam);
  },
  created() {
    this.$root.$on('search-results', this.receiveSearchParam);
  },
  beforeDestroy() {
    this.$root.$off('search-results', this.receiveSearchParam);
  },
};
</script>

<style scoped>
.btn-center {
  display: block;
  margin: 0 auto;
}
</style>
