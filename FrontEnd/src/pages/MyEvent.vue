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
                    {{ calculateTime1(event.start_at) }}
                </div>
                <div slot="footer" class="text-muted">
                    {{ calculateTime2(event.end_at) }}
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
            currentUserId: localStorage.getItem('id'), // 从 localStorage 读取 currentUserId
            token: localStorage.getItem('token'), // 从 localStorage 读取 token
            eventData: [], // 将 eventData 定义为数组
        }
    },
    mounted() {
        this.fetchEvents();
    },
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
        async fetchEvents() {
            try {
                const response = await axios.get('https://backend.sustech.me/api/event/participated', {
                headers: {
                    Authorization: `Bearer ${this.token}`
                },
                params: {
                    page: 1
                }
                })

                this.eventData = response.data.events
            } catch (error) {
                console.error('Error fetching chats:', error)
            }
        },
        calculateTime1(start) {
            // const currentTime = new Date();
            // const startDate = new Date(start);
            // const endDate = new Date(end);
           
            return "Start Time: " + start;
        },
        calculateTime2(start) {
            // const currentTime = new Date();
            // const startDate = new Date(start);
            // const endDate = new Date(end);
           
            return "End Time: " + start;
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
