<style>
    .notification {
        position: absolute;
        top: 17px;
        right: 120px;
        width: 6px;
        height: 6px;
    }
</style>

<template>
    <div>
        <div class="row">
            <!-- <card v-for="event in eventData" :key="event.id" type="nav-tabs" class="text-center"></card> -->
            <card v-for="event in eventData" :key="event.id" type="nav-tabs" class="text-center">
                <div slot="header" class="card-header-primary">
                    Hold by {{ event.organizer.name }}
                </div>
                <h4 class="card-title">{{ event.name }}</h4>
                <p class="card-text">{{ event.description }}</p>
                <!-- <a href="#" class="btn btn-primary">Check</a> -->
                <a :href="'#/event/' + event.id" class="btn btn-primary">Check</a>
                <div slot="footer" class="text-muted">
                    {{ calculateTime(event.start_at) }}
                </div>
            </card>
        </div>
        <!-- Notification Icon with Conditional Rendering -->
        <a slot="title" class="dropdown-toggle nav-link" data-toggle="dropdown" aria-expanded="true">
            <i class="tim-icons icon-bell-55"></i>
            <span v-if="upcomingEventsCount > 0" class="notification"></span>
            <!-- <p class="d-lg-none">New Notifications</p> -->
        </a>
    </div>
</template>
    
<script>
import axios from 'axios';

export default {
    data() {
        return {
            eventData: [], // 将 eventData 定义为数组
            }

        // return {
        //     eventData: [
        //         {
        //             id: 1,
        //             holder: "holder 1",
        //             title: "title 1",
        //             content: "content 1",
        //             time: "2024-05-31",
        //         },
        //         {
        //             id: 2,
        //             holder: "holder 2",
        //             title: "title 2",
        //             content: "content 2",
        //             time: "2024-05-20",
        //         },
        //     ]
        // }
    },
    mounted() {
        this.i18n = this.$i18n;
        if (this.enableRTL) {
        this.i18n.locale = "ar";
        this.$rtl.enableRTL();
        }
        const list_event_api = 'https://backend.sustech.me/api/event'
        axios.get(list_event_api, {
        headers: {
        }
        })
            .then(response => {
            const eventData = response.data;
            this.eventData = eventData.events;
            // this.eventData.organizer = eventData.events.organizer;
            })
            .catch(error => {
            console.error('Error fetching profile data:', error);
            });
    },
    // mounted() {
    //     this.token = localStorage.getItem('token');
    //     if (!this.token) {
    //         console.log("Token not found.");
    //         return;
    //     }

    //     this.eventData.id = 2;
    //     const apiUrl = `https://backend.sustech.me/api/event/${this.eventData.id}`; // 修正 eventId 

    //     axios.get(apiUrl, {
    //         headers: {
    //         }
    //     })
    //     .then(response => {
    //         const eventData = response.data;
    //         this.eventData.push({
    //             id: eventData.id,
    //             name: eventData.name,
    //             description: eventData.description,
    //             organizer: eventData.organizer,
    //             start_at: eventData.start_at,
    //             end_at: eventData.end_at,
    //         });
    //     })
    //     .catch(error => {
    //         console.error('Error fetching profile data:', error);
    //     });
    // },
    computed: {
        upcomingEventsCount() {
            const currentTime = new Date();
            return this.eventData.filter(event => {
                const eventDate = new Date(event.time);
                const timeDiff = eventDate.getTime() - currentTime.getTime();
                const daysDiff = Math.ceil(timeDiff / (1000 * 3600 * 24));
                return daysDiff <= 3;
            }).length;
        }
    },
    methods: {
        calculateTime(eventTime) {
            const currentTime = new Date();
            const eventDate = new Date(eventTime);
            const timeDiff = eventDate.getTime() - currentTime.getTime();
            const daysDiff = Math.ceil(timeDiff / (1000 * 3600 * 24)); // Convert milliseconds to days
            if (daysDiff <= 3) { // Check if the event is within 3 days
                return "Happening soon! " + (daysDiff === 1 ? "tomorrow" : "in " + daysDiff + " days");
            } else {
                return "in " + daysDiff + " days";
            }
        },
        getEventById(eventId) {
        axios.get(`https://backend.sustech.me/api/event/${eventId}`) // 发送 GET 请求到指定 eventId 的事件数据接口
            .then(response => {
                // 更新特定 eventId 的事件数据
                const eventData = response.data;
                const existingEventIndex = this.eventData.findIndex(event => event.id === eventId);
                if (existingEventIndex !== -1) {
                    this.$set(this.eventData, existingEventIndex, eventData);
                }
            })
            .catch(error => {
                console.error('Error fetching event data:', error);
            });
    }
    },
}
</script>
