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
            <card v-for="event in eventData" :key="event.id" type="nav-tabs" class="text-center">
                <div slot="header" class="card-header-primary">
                    Hold by {{ event.holder }}
                </div>
                <h4 class="card-title">{{ event.title }}</h4>
                <p class="card-text">{{ event.content }}</p>
                <!-- <a href="#" class="btn btn-primary">Check</a> -->
                <a :href="'#/event/' + event.id" class="btn btn-primary">Check</a>
                <div slot="footer" class="text-muted">
                    {{ calculateTime(event.time) }}
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
export default {
    data() {
        return {
            eventData: [
                {
                    id: 1,
                    holder: "holder 1",
                    title: "title 1",
                    content: "content 1",
                    time: "2024-05-31",
                },
                {
                    id: 2,
                    holder: "holder 2",
                    title: "title 2",
                    content: "content 2",
                    time: "2024-05-20",
                },
            ]
        }
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
        }
    },
}
</script>
