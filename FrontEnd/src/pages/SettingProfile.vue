<template>
    <card>
        <h5 slot="header" class="title">Edit Profile</h5>
        <div class="row">
            <div class="col-md-12">
                <input type="file" @change="handleFileUpload" style="display: none"
                    ref="fileInput" accept=".jpg, .jpeg, .png, .gif">
                <img class="avatar" :src="imageUrl" alt="User Avatar" style="width: 200px;
                    height: 200px;" @click="openFileInput"/>
            </div>
        </div>
        <div class="row">
            <div class="col-md-5 pr-md-1">
                <base-input
                    label="Role"
                    placeholder="role"
                    v-model="user.role"
                    disabled
                >
                </base-input>
            </div>
            <div class="col-md-3 px-md-1">
                <base-input
                    label="Name"
                    placeholder="name"
                    v-model="user.name"
                    disabled
                >
                </base-input>
            </div>
            <div class="col-md-4 pl-md-1">
                <base-input
                    label="SID"
                    placeholder="sid"
                    v-model="user.sid"
                    disabled
                >
                </base-input>
            </div>
        </div>
        <div class="row">
            <div class="col-md-12">
                <base-input
                    label="Email"
                    type="email"
                    :placeholder="user.email"
                    v-model="user.email_edited"
                >
                </base-input>
            </div>
        </div>
        <div class="row">
            <div class="col-md-8">
                <base-input>
                    <label>About Me</label>
                    <textarea
                        rows="4"
                        cols="80"
                        class="form-control"
                        :placeholder="user.description ? user.description : 'This is your description'"
                        v-model="user.description_edited"
                    >
                    </textarea>
                </base-input>
            </div>
        </div>
        <base-button slot="footer" type="primary" fill @click="saveProfile">Save</base-button>
        <base-alert v-if="showSuccessAlert" type="success">
            <strong>Success!</strong> Profile updated successfully.
        </base-alert>
        <base-alert v-if="showErrorAlert" type="danger">
            <strong>Error!</strong> Failed to update profile. Error Message: {{ errorMessage }}. 
            Your token is: {{token}}
        </base-alert>
    </card>
</template>
  
<script>
import axios from 'axios';

export default {
    data() {
        return {
            imageUrl: '',
            user: {
                id: null,
                sid: null,
                name: null,
                role: null,
                email: null,
                email_edited: null,
                avatar: null,
                avatar_edited: null,
                description: null,
                description_edited: null
            },
            token: null,
            showSuccessAlert: false,
            showErrorAlert: false,
            errorMessage: '',
            fileToUpload: null
        };
    },
    mounted() {
        this.token = localStorage.getItem('token');
        if (!this.token) {
            console.log("Token not found.");
            return;
        }

        this.user.id = localStorage.getItem('id');
        const apiUrl = `https://backend.sustech.me/api/account/${this.user.id}/profile`;

        axios.get(apiUrl, {
            headers: {
                Authorization: `Bearer ${this.token}`
            }
        })
        .then(response => {
            const userData = response.data;
            this.user.sid = userData.sustech_id;
            this.user.name = userData.name;
            this.user.role = userData.role;
            this.user.email = userData.email;
            this.user.avatar = userData.avatar;
            this.user.description = userData.bio;
            this.imageUrl = `https://backend.sustech.me/uploads/${userData.avatar}.webp`;
        })
        .catch(error => {
            console.error('Error fetching profile data:', error);
        });
    },
    methods: {
        saveProfile() {
            if (this.fileToUpload) {
                this.uploadFile();
            } else {
                this.updateProfile();
            }
        },
        openFileInput() {
            this.$refs.fileInput.click();
        },
        handleFileUpload(event) {
            this.fileToUpload = event.target.files[0];
            const reader = new FileReader();
            reader.readAsDataURL(this.fileToUpload);
            reader.onload = (event) => {
                this.imageUrl = event.target.result;
            };
        },
        uploadFile() {
            const formData = new FormData();
            formData.append('file', this.fileToUpload);

            axios.post('https://backend.sustech.me/upload', formData, {
                headers: {
                    'Content-Type': 'multipart/form-data'
                }
            })
            .then(response => {
                console.log('File uploaded successfully:', response.data);
                const filename = response.data.split('/').pop().replace('.webp', '');
                this.user.avatar_edited = filename;
                this.updateProfile();
            })
            .catch(error => {
                console.error('Error uploading file:', error);
            });
        },
        updateProfile() {
            const apiUrl = `https://backend.sustech.me/api/account/${this.user.id}/profile`;
            const requestBody = {
                bio: this.user.description_edited ? this.user.description_edited : this.user.description,
                avatar: this.user.avatar_edited ? this.user.avatar_edited : this.user.avatar
            };
            if (this.user.email_edited) {
                Object.assign(requestBody, { email: this.user.email_edited });
            }

            axios.put(apiUrl, requestBody, {
                headers: {
                    Authorization: `Bearer ${this.token}`
                }
            })
            .then(response => {
                console.log('Profile updated successfully:', response.data);
                this.showSuccessAlert = true;
                this.showErrorAlert = false;
                location.reload();
            })
            .catch(error => {
                console.error('Error updating profile:', error);
                console.log('Edited avatar: ', this.user.avatar_edited);
                console.log('Edited email: ', this.user.email_edited);
                this.showSuccessAlert = false;
                this.showErrorAlert = true;
                this.errorMessage = error.response.data.message;
            });
        }
    }
}
</script>

<style></style>
