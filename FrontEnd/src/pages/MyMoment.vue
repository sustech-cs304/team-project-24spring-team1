<style>
.image-container {
    display: flex;
    justify-content: center; /* 水平居中 */
    align-items: center; /* 垂直居中 */
    width: 100%; 
    height: 100%; 
}
.card-text {
    margin-bottom: 20px;
}
.name {
    font-size: 20px; 
}
.role {
    margin-left: 5px; 
}
.created-time {
    float: right; 
}
.comment-container {
    text-align: center; 
}

.comment-input-wrapper {
    display: flex;
    align-items: center;
    width: 100%;
}

.comment-button {
    margin-left: 20px; 
}

</style>
<template>
    <div>
        <div class="row">
            <div class="col-lg-4" :class="{ 'text-right': isRTL }">
                <card v-for="(card, index) in moments" :key="index" style="width: 50rem;">
                    <!-- <p :style="{ color: 'black' }">Moment ID: {{ card.id }}</p> -->
                    <div class="card-text">
                        <span class="name">{{ card.account.name }}</span> | <span class="role">{{ card.account.role }}</span>
                    </div>
                    
                    <div class="card-text">{{ card.content }}</div>

                    <template v-for="(imageNumber, i) in [1]">
                        <div class="image-container">
                            <img slot="image" class="card-img-top" :key="i" :src="getMomentImagePath(card.id, imageNumber)" alt="Card image cap" style="width: 400px; height: auto;"/>
                        </div>
                    </template>

                    <div class="card-text created-time">{{ card.created_at }}</div>


                    <div class="comment-container">
                        <div style="margin-bottom: 40px;"></div> <!-- 空两行 -->
                        <div class="comment-input-wrapper">
                            <input type="text" class="form-control" v-model="newComment" placeholder="Comment here..." style="color: black;">
                            <base-button class="animation-on-hover" style="margin-left: 10px;" @click="postComment(card.id)">Comment</base-button>
                        </div>
                    </div>

                    <div>
                        <base-button class="animation-on-hover" type="success" @click="getComments(card.id)">
                            <i class="tim-icons icon-chat-33" style="margin-right: 5px;"></i> Show Comments
                        </base-button>
                        <div v-for="(comment, cIndex) in comments" :key="cIndex">
                            <card class="mb-3">
                                <h4 class="card-title">{{ comment.account.name }}</h4>
                                <p class="card-text">{{ comment.content }}</p>
                                <p class="card-text"><small class="text-muted">{{ comment.created_at }}</small></p>
                            </card>
                        </div>
                    </div>

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
        

        <div class="comment-container">
            <div style="margin-bottom: 40px;"></div>
            <div class="comment-input-wrapper">
                <input type="text" class="form-control" v-model="newMoment" placeholder="New moment content here..." style="color: black;">
            </div>
        </div>
        <!-- Post Moment Button -->
        <div class="row mt-3">
            <div class="col-lg-4">
                <button @click="postNewMoment" class="btn btn-primary">Post Moment</button>
            </div>
        </div>
    </div>
</template>


<script>
import axios from 'axios';
export default {
    props: ['username'],
    data() {
        return {
            moments: [],
            comments: [],
            idx:0,
            momentID: null,
            userName:null,
            content: null,
            time: null,
            newComment: '',
            newMoment: '',
            buttons:[
                { icon: "tim-icons icon-heart-2" },
                { icon: "tim-icons icon-chat-33" }, 
            ]
        }
    },
    mounted() {       
        this.fetchMoments(); // 在组件挂载时,调用fetchMoments方法获取数据
    },
    methods: {
        postNewMoment() {
            const commentUrl = `https://backend.sustech.me/api/moment`;
            const commentData = {
                content: this.newMoment
            };

            this.token = localStorage.getItem('token');
            if (!this.token) {
                console.log("Token not found.");
                return;
            }

            axios.post(commentUrl, commentData, {
                headers: {
                Authorization: `Bearer ${this.token}`
                }
            })
                .then(response => {
                    // Handle successful comment submission
                    alert('Moment Post successfully.');
                    
                    this.newMoment = ''; 
                    const newMomentId = response.data.id;
                    
                    console.log('New Moment ID:', newMomentId);
                })
                .catch(error => {
                    console.error('Error submitting comment:', error);
                });
        },
        fetchMoments() {
            const commentUrl = `https://backend.sustech.me/api/moment`;
            axios.get(commentUrl)
                .then(response => {
                    // this.momentID = response.data.moments.id;
                    this.moments = response.data.moments;
                    // this.momentID = this.moments[this.idx].id;
                    // this.idx++;                   
                })
                .catch(error => {
                    console.error('Error fetching comments:', error);
                });
        },

        postComment(id) {
            const commentUrl = `https://backend.sustech.me/api/moment/${id}/comment`;
            const commentData = {
                content: this.newComment
            };

            this.token = localStorage.getItem('token');
            if (!this.token) {
                console.log("Token not found.");
                return;
            }

            axios.post(commentUrl, commentData, {
                headers: {
                Authorization: `Bearer ${this.token}`
                }
            })
                .then(response => {
                    // Handle successful comment submission
                    alert('Comment submitted successfully.');
                    this.newComment = ''; // Clear the comment input
                    // this.getComments(id); // Refresh the comments list
                })
                .catch(error => {
                    console.error('Error submitting comment:', error);
            });
        },

        getComments(id) {
            const commentUrl = `https://backend.sustech.me/api/moment/${id}/comment`;
            axios.get(commentUrl)
                .then(response => {
                    this.comments = response.data.comments;
                })
                .catch(error => {
                    console.error('Error fetching comments:', error);
            });
        },

        getMomentImagePath(momentId, imageNumber) {
            return `users/testuser/moment/${momentId}-${imageNumber}.jpg`;
        },
    }
}
</script>
