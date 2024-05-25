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
.moment-container {
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
                        <a href="#/dashboard/profile" @click="handleClick(card.account.id)">
                            <span class="name">{{ card.account.name }}</span> | <span class="role">{{ card.account.role }}</span>
                        </a>
                    </div>
                    
                    <div class="card-text">{{ extractContent(card.content) }}</div>
                    <!-- <div class="card-text">content: {{ card.content }}</div> -->
                    <div class="image-container">
                        <img :src="extractImageLink(card.content)" style="width: 400px; height: auto;"/>    
                        
                    </div>

                    <!-- <img :src="extractImageLink(card.content)"/>
                     -->
                    <!-- <img :src="'https://backend.sustech.me/uploads/273d5fb1-558b-440a-a1d9-3021e13599d4.webp'" alt="Uploaded Image" /> -->
                    <!-- <img :src="imageUrl" alt="Uploaded Image" v-if="imageUrl" /> -->
                    
                    
                    <!-- <template v-for="(imageNumber, i) in [1]">
                        <div class="image-container">
                            <img slot="image" class="card-img-top" :key="i" :src="getMomentImagePath(card.id, imageNumber)" style="width: 400px; height: auto;"/>
                        </div>
                    </template> -->

                    <div class="card-text created-time">{{ formatDate(card.created_at) }}</div>

                    <div class="comment-container">
                        <div style="margin-bottom: 40px;"></div> 
                        <div class="comment-input-wrapper">
                            <input type="text" class="form-control" v-model="newComments[card.id]" placeholder="Comment here..." style="color: black;">
                            <base-button class="animation-on-hover" style="margin-left: 10px;" @click="postComment(card.id)">Shoot!</base-button>
                        <base-button class="animation-on-hover" type="success" @click="getComments(card.id)">
                            <!-- <i class="tim-icons icon-chat-33" style="margin-right: 5px;"></i> Show Comments -->
                            <i class="tim-icons icon-chat-33" ></i>
                            {{ showComments[card.id] ? 'Hide' : 'Show' }}
                        </base-button>
                        </div>
                    </div>
                    <div>
            
                        <div v-if="showComments[card.id]">
                            
                            <div v-if="momentComments[card.id]" v-for="(comment, cIndex) in momentComments[card.id]" :key="cIndex">
                                <card class="mb-3">
                                <a href="#/dashboard/profile" @click="handleClick(comment.account.id)">{{ comment.account.name }}</a>
                                <p class="card-text">{{ comment.content }}</p>
                                <p class="card-text"><small class="text-muted">{{ comment.created_at }}</small></p>
                                </card>
                            </div>
                        </div>           

                    </div>
                </card>
            </div>
        </div>
        

        <!-- <div class="comment-container">
            <div style="margin-bottom: 40px;"></div>
            <div class="comment-input-wrapper">
                <input type="text" class="form-control" v-model="newMoment" placeholder="New moment content here..." style="color: black;">
                <input type="file" @change="handleImageUpload" accept="image/*" />
                
            </div>           
        </div> -->
        <div class="comment-container">
            <div style="margin-bottom: 40px;"></div>
            <div class="comment-input-wrapper">
                <input type="text" class="form-control" v-model="newMoment" placeholder="New moment content here..." style="color: black;">
                <input type="file" @change="handleImageUpload" accept="image/*" />
                <!-- <img :src="imageUrl" alt="Uploaded Image" v-if="imageUrl" /> -->
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
import marked from 'marked';
export default {
    props: ['username'],
    data() {
        return {
            moments: [],
            momentComments: {},
            showComments: {},
            comments: [],
            momentID: null,
            avatar: null,
            newComments: {},
            newMoment: '',
            selectedImage: null,
            imageUrl: '',
            renderedContent: {},
            buttons:[
                { icon: "tim-icons icon-heart-2" },
                { icon: "tim-icons icon-chat-33" }, 
            ]
        }
    },
    mounted() {       
        this.fetchMoments(); // 在组件挂载时,调用fetchMoments方法获取数据
        // this.extractImageUrl(this.moments.content);
    },
    methods: {
        extractContent(content) {
            const match = content.match(/```(.*?)```/);
            if (match) {
                return content.replace(match[0], '').trim();
            } else {
                return content;
            }
        },
        extractImageLink(content) {
            // 使用正表达式匹配链接
            const match = content.match(/```(.*?)```/);
            if (match && match.length > 1) {
                return match[1];
            } else {
                return '';
            }
        },

        handleImageUpload(event) {
            this.selectedImage = event.target.files[0];
            
            const formData = new FormData();
            formData.append('file', this.selectedImage);

            axios.post('https://backend.sustech.me/upload', formData, {
                headers: {
                'Content-Type': 'multipart/form-data',
                Authorization: `Bearer ${this.token}`
                }
            })

            .then(response => {
                const fileUrl = 'https://backend.sustech.me' + response.data
                console.log('File uploaded successfully:', response.data)
                // 将图片 URL 存储在 momentImages 对象中
                //this.momentImages[this.newMoment] = fileUrl;
                this.imageUrl = fileUrl

                // 将文件 URL 添加到 this.newMoment 中
                this.newMoment += '\n\n' + '```' + fileUrl + '```';
                
            })
            .catch(error => {
                console.error('Error uploading file:', error);
            });
        },



        handleClick(newID) {
            console.log('set profile ID =', newID)
            localStorage.setItem('profileCurrentID', newID);
            window.location.href = '#/dashboard/profile';
        },

        getComments(id) {
            const commentUrl = `https://backend.sustech.me/api/moment/${id}/comment`;

            // Toggle the showComments value
            this.$set(this.showComments, id, !this.showComments[id]);

            axios.get(commentUrl)
                .then(response => {
                this.$set(this.momentComments, id, response.data.comments);
                })
                .catch(error => {
                console.error('Error fetching comments:', error);
                });
        },
        postNewMoment() {
            const commentUrl = `https://backend.sustech.me/api/moment`;
            const commentData = {
                content: this.newMoment,               
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
                    // Handle successful post
                    alert('Moment Post successfully.');
                    
                    this.newMoment = ''; 
                    const newMomentId = response.data.id;

                    console.log('New Moment ID:', newMomentId);
                })
                .catch(error => {
                    console.error('Error posting moments:', error);
                });
        },

        fetchMoments() {
            const commentUrl = `https://backend.sustech.me/api/moment`;
            axios.get(commentUrl)
                .then(response => {
                    // this.momentID = response.data.moments.id;
                    this.moments = response.data.moments;                 
                })
                .catch(error => {
                    console.error('Error fetching comments:', error);
                });
        },

        postComment(id) {
            const commentUrl = `https://backend.sustech.me/api/moment/${id}/comment`;
            const commentData = {
                content: this.newComments[id]
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
                this.$set(this.newComments, id, ''); // Clear the comment input
                this.getComments(id); // Refresh the comments list
            })
            .catch(error => {
                console.error('Error submitting comment:', error);
            });
        },
        getMomentImagePath(momentId, imageNumber) {
            return `users/testuser/moment/${momentId}-${imageNumber}.jpg`;
        },

        formatDate(dateString) {
            const date = new Date(dateString);
            const options = {
                year: 'numeric',
                month: 'long',
                day: 'numeric',
                hour: '2-digit',
                minute: '2-digit',
                second: '2-digit'
            };
            return date.toLocaleDateString('en-US', options);
        },
    }
}
</script>
