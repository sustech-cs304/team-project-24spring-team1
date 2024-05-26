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
    </div>
</template>
    
<script>
/**
     * AI-generated-content
     * tool: ChatGPT
     * version: 3.5
     * usage: I used the prompt "generate event page", and
     * directly copy the code from its response
     */
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
            return "Start Time: " + this.formatDate(start);
        },
        calculateTime2(end) {
            return "End Time: " + this.formatDate(end);
        },
        formatDate(dateString) {
            const date = new Date(dateString);
            const options = { year: 'numeric', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' };
            return date.toLocaleDateString('en-US', options);
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
