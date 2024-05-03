<template>
    <card>
        <h5 slot="header" class="title">Edit Profile</h5>
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
                    :placeholder="user.sid + '@mail.sustech.edu.cn'"
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
                        v-model="user.description"
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
            <strong>Error!</strong> Failed to update profile. {{ errorMessage }}
        </base-alert>
    </card>
</template>
  
<script>
import axios from 'axios';

export default {
    data() {
        return {
            user: {
                id: null,
                sid: null,
                name: null,
                role: null,
                description: null
            },
            token: null,
            showSuccessAlert: false,
            showErrorAlert: false,
            errorMessage: ''
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
            this.user.sid = 12111611;
            this.user.name = userData.name;
            this.user.role = userData.role;
            this.user.description = userData.bio;
        })
        .catch(error => {
            console.error('Error fetching profile data:', error);
        });
    },
    methods: {
        saveProfile() {
            const apiUrl = 'https://backend.sustech.me/api/account/self/profile';
            const requestBody = {
                bio: this.user.description
            };

            axios.put(apiUrl, requestBody, {
                headers: {
                    Authorization: `Bearer ${this.token}`
                }
            })
            .then(response => {
                console.log('Profile updated successfully:', response.data);
                this.showSuccessAlert = true;
                this.showErrorAlert = false;
            })
            .catch(error => {
                console.error('Error updating profile:', error);
                this.showSuccessAlert = false;
                this.showErrorAlert = true;
                this.errorMessage = error.response.data.message;
            });
        }
    }
}
</script>

<style></style>
