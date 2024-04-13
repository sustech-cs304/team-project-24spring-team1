<template>
    <div>
        <div class="row">
            <card v-for="event in eventData" :key="event.id" type="nav-tabs" class="text-center">
                <div slot="header" class="card-header-primary">
                    Hold by {{ event.holder }}
                </div>
                <h4 class="card-title">{{ event.title }}</h4>
                <p class="card-text">{{ event.content }}</p>
                <a href="#" class="btn btn-primary">Check</a>
                <div slot="footer" class="text-muted">
                    {{ calculateTime(event.time) }}
                </div>
            </card>
        </div>
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
                    time: "2024-05-02",
                },
                {
                    id: 2,
                    holder: "holder 2",
                    title: "title 2",
                    content: "content 2",
                    time: "2024-05-03",
                },
            ]
        }
    },
    methods: {
        calculateTime(eventTime) {
            const currentTime = new Date();
            const eventDate = new Date(eventTime);
            const timeDiff = eventDate.getTime() - currentTime.getTime();
            const daysDiff = Math.ceil(timeDiff / (1000 * 3600 * 24)); // Convert milliseconds to days
            if (daysDiff === 1) {
                return "in 1 day";
            } else {
                return "in " + daysDiff + " days";
            }
        }
    }
}
</script>
