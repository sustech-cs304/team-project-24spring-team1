<template>
    <div>
        <div class="row">
            <div class="col-lg-4" :class="{ 'text-right': isRTL }">
                <card v-for="(card, index) in moment" :key="index" style="width: 50rem;">
                    <img slot="image" class="card-img-top" :src="getMomentImagePath(card.id)" alt="Card image cap"/>
                    <p class="card-text">{{ card.content }}</p>
                    <div class="d-flex justify-content-around">
                        <base-button v-for="(button, bIndex) in card.buttons" :key="bIndex" round icon type="primary">
                            <i :class="button.icon"></i>
                        </base-button>
                    </div>
                    <div class="dropdown-divider"></div>
                    <div v-for="(comment, cIndex) in card.comments" :key="cIndex">
                        <card class="mb-3">
                            <h4 class="card-title">{{ comment.username }}</h4>
                            <p class="card-text">{{ comment.comment }}</p>
                            <p class="card-text"><small class="text-muted">{{ comment.time }}</small></p>
                        </card>
                    </div>
                </card>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    data() {
        return {
            moment: [
                {
                    id: 1,
                    content: "Content 1",
                    buttons: [
                        { icon: "tim-icons icon-heart-2" },
                        { icon: "tim-icons icon-chat-33" },
                        { icon: "tim-icons icon-simple-delete" }
                    ],
                    comments: [
                        { username: "User 1", comment: "This is a wider card...", time: "Last updated 1 mins ago" },
                        { username: "User 2", comment: "This is a wider card...", time: "Last updated 10 mins ago" }
                    ]
                },
                {
                    id: 2,
                    content: "Content 2",
                    buttons: [
                        { icon: "tim-icons icon-heart-2" },
                        { icon: "tim-icons icon-chat-33" },
                        { icon: "tim-icons icon-simple-delete" }
                    ],
                    comments: [
                        { username: "User 3", comment: "This is a wider card...", time: "Last updated 2 mins ago" },
                        { username: "User 4", comment: "This is a wider card...", time: "Last updated 5 mins ago" }
                    ]
                }
            ]
        }
    },
    methods: {
        getMomentImagePath(momentId) {
            return `Data/user/moment/${momentId}/pictures/image.jpg`;
        }
    }
}
</script>
