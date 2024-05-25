<template>
  <div class="login-container">
    <el-form ref="loginForm" :model="loginForm" :rules="loginRules" class="login-form" auto-complete="on" label-position="left">

      <div class="title-container">
        <h3 class="title">Welcome to SUSTech Event!</h3>
      </div>

      <el-form-item prop="userName">
        <span class="svg-container">
          <svg-icon icon-class="user" />
        </span>
        <el-input
          ref="userName"
          v-model="loginForm.userName"
          placeholder="email"
          name="userName"
          type="text"
          tabindex="1"
          auto-complete="on"
        />
      </el-form-item>

      <el-form-item prop="password">
        <span class="svg-container">
          <svg-icon icon-class="password" />
        </span>
        <el-input
          :key="passwordType"
          ref="password"
          v-model="loginForm.password"
          :type="passwordType"
          placeholder="password"
          name="password"
          tabindex="2"
          auto-complete="on"
          @keyup.enter.native="handleLogin"
        />
        <span class="show-pwd" @click="showPwd">
          <svg-icon :icon-class="passwordType === 'password' ? 'eye' : 'eye-open'" />
        </span>
      </el-form-item>

      <el-form-item>
        <div class="button-container">
          <el-button :loading="loading" type="primary"
                     @click.native.prevent="visitor_login">visitor LOGIN</el-button>

          <el-button :loading="loading" type="primary"
                     @click.native.prevent="handleLogin">go to CAS login</el-button>
        </div>
      </el-form-item>


<!--      <p class="tips">-->
<!--        <a href="#/register" type="primary">no account? register now.</a>-->
<!--      </p>-->

      <div>
        <mouseClick>
          <a href="#/register" type="primary">no account? register now.</a>
        </mouseClick>
      </div>

    </el-form>
  </div>
</template>

<script>
import axios from 'axios';
import api from "js-cookie";
import mouseClick from "@/adminform/components/mouseClick.vue";

// import {debug} from "script-ext-html-webpack-plugin/lib/common";

export default {
  data() {
    return {
      loginForm: {
        userName: '',
        password: '',
      },
      ifAdmin: false,
      loginRules: {
        userName: [ // type: 'email',
          { required: true, trigger: 'blur', message: 'please enter your id' },
          // {
          //   validator: (rule, value, callback) => {
          //     if (value && value.endsWith('admin.com')) {
          //       this.ifAdmin = true;
          //     }
          //     callback(); // 继续验证流程
          //   }
          // },
        ],
        password: [{
          required: true,
          message: 'enter password',
          trigger: 'blur'
        },]
          // { pattern: /^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,20}$/, message: 'password has to contain both letters and digits, and length 8-20.' }]
      },
      loading: false,
      passwordType: 'password',
      redirect: undefined,
      // token:'',
    }
  },
  watch: {
    $route: {
      handler: function(route) {
        this.redirect = route.query && route.query.redirect
      },
      immediate: true
    }
  },
  methods: {
    showPwd() {
      if (this.passwordType === 'password') {
        this.passwordType = ''
      } else {
        this.passwordType = 'password'
      }
      this.$nextTick(() => {
        this.$refs.password.focus()
      })
    },
    visitor_login(){
      const apiUrl = `https://backend.sustech.me/api/auth/login`;
      const requestData = {
        sustech_id: parseInt(this.loginForm.userName),
        password: this.loginForm.password
      };
      axios.post(apiUrl, requestData)
          .then(response => {
            const token = response.data.token;
            const account_id = response.data.account_id;
            localStorage.setItem('token',token);
            localStorage.setItem('id',account_id);
            axios.get(`https://backend.sustech.me/api/account/${account_id}/profile`, {
            headers: {
              Authorization: `Bearer ${token}`
            }
          })
            .then(profileResponse => {
              const avatarUUID = profileResponse.data.avatar;
              localStorage.setItem('imageUrl', "https://backend.sustech.me/uploads/" + avatarUUID + ".webp");

              this.showSuccessMessage("登录成功");
              this.$refs.loginForm.validate(valid => {
                if (valid) {
                  let path = '/Dashboard/dashboard';
                  if (this.ifAdmin) {
                    path = '/admin/publish';
                  }
                  this.$router.push({ path: path });
                }
              });
            })
            .catch(profileError => {
              console.error('Failed to fetch profile:', profileError);
              this.showFailMessage('登录失败');
            });
          })
          .catch(error => {
            // console.error("data:", eventData);
            console.error('failed to publish event:', error);
            this.showFailMessage(`登录失败`);
          })
    },
    async handleLogin(){
        // Step 1: Make a request to /api/auth/identifier to get an identifier
        const response = await axios.get('https://backend.sustech.me/api/auth/identifier');
        const identifier = response.data.identifier;
        // Step 2: Open a new tab in the browser for login
        const loginUrl = `https://sso.cra.ac.cn/realms/cra-service-realm/protocol/cas/login?service=https://backend.sustech.me/api/auth/callback?identifier=${identifier}`;
        window.open(loginUrl, '_blank');
        // Step 3: Make a request to /api/auth/poll with the identifier obtained in step 1
        this.checkLoginStatus(identifier);
      },
      async checkLoginStatus(identifier) {
        try {
          const response = await axios.get(`https://backend.sustech.me/api/auth/poll?identifier=${identifier}`);
          const userData = response.data;
          localStorage.setItem('id', userData.account_id);
          localStorage.setItem('token', userData.token);
          if (userData.token && userData.account_id) {
            // clearInterval(pollInterval);
            this.showSuccessMessage("登录成功");
            this.$refs.loginForm.validate(valid => {
              if (valid) {
                let path = '/Dashboard/dashboard';
                if (this.ifAdmin) {
                  path = '/admin/publish';
                }
                this.$router.push({path: path});
              }
            });}
          else { // Login process is not completed, continue polling
            setTimeout(() => {
              this.checkLoginStatus(identifier); // Polling for login status
            }, 40000); // Polling interval: 5 seconds
          }
        } catch (error) {
          this.showFailMessage("登录失败! ");
        }
        },

    showSuccessMessage(message) {
      this.successMessage = message;
      this.$message({
        message: this.successMessage,
        type: 'success'
      });
    },
    showFailMessage(message) {
      this.Message = message;
      this.$message({
        message: this.Message,
        type: 'error'
      });
    },

  }
}
</script>

<style lang="scss">
/* 修复input 背景不协调 和光标变色 */
/* Detail see https://github.com/PanJiaChen/vue-element-admin/pull/927 */

$bg:#283443;
$light_gray:#fff;
$cursor: #fff;

@supports (-webkit-mask: none) and (not (cater-color: $cursor)) {
  .login-container .el-input input {
    color: $cursor;
  }
}

/* reset element-ui css */
.login-container {
  .el-input {
    display: inline-block;
    height: 90%; /*47px;*/
    width: 85%;

    input {
      background: transparent;
      border: 0px;
      -webkit-appearance: none;
      border-radius: 0px;
      padding: 12px 5px 12px 15px;
      color: $light_gray;
      height: 47px;
      caret-color: $cursor;

      &:-webkit-autofill {
        box-shadow: 0 0 0px 1000px $bg inset !important;
        -webkit-text-fill-color: $cursor !important;
      }
    }
  }

  .el-form-item {
    border: 1px solid rgba(218, 128, 128, 0.1);
    background: rgba(0, 0, 0, 0.1);
    border-radius: 5px;
    color: #454545;
  }
}
</style>

<style lang="scss" scoped>
$bg: #edf0f3;
$dark_gray: #f6f7f8;
$light_gray: #eee;
$cursor:white;

body {
  min-height: 100%;
  width: 100%;
  background-color: $bg;
  overflow: hidden;
}

.login-container {
  position: relative; /* 必须设置为相对定位或其他定位才能使用伪元素 */
  min-height: 100vh; /* 使容器至少和视口高度一样 */
  width: 100%;

  &:before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-image: url('@/adminform/assets/sustech.png');
    background-size: cover; /* 使背景图片覆盖整个容器 */
    background-position: center; /* 背景图片居中对齐 */
    background-repeat: no-repeat; /* 防止背景图片重复 */
    opacity: 0.8; /* 设置透明度 */
    z-index: -1; /* 使伪元素在其他内容的下面 */
  }
}

.el-input {
  display: inline-block;
  height: 90%; /* 47px; */
  width: 85%;

  input {
    background: transparent;
    border: 0px;
    -webkit-appearance: none;
    border-radius: 0px;
    padding: 12px 5px 12px 15px;
    color: $light_gray;
    height: 47px;
    caret-color: $cursor;

    &:-webkit-autofill {
      box-shadow: 0 0 0px 1000px $bg inset !important;
      -webkit-text-fill-color: $cursor !important;
    }
  }
}

.el-form-item {
  border: 1px solid rgba(218, 128, 128, 0.1);
  background: rgba(0, 0, 0, 0.1);
  border-radius: 5px;
  color: #454545;
}

.login-form {
  position: relative;
  width: 520px;
  max-width: 100%;
  padding: 160px 35px 0;
  margin: 0 auto;
  overflow: hidden;
}

.button-container {
  display: flex;
  justify-content: space-between;
  gap: 20px; /* 设置按钮之间的间隔 */
}

.button-container .el-button {
  flex: 1;
}

.tips {
  float: right;
  font-size: 14px;
  color: #fff;
  margin-bottom: 10px;

  span {
    &:first-of-type {
      margin-right: 16px;
    }
  }
}

.svg-container {
  padding: 6px 5px 6px 15px;
  color: $dark_gray;
  vertical-align: middle;
  width: 30px;
  display: inline-block;
}

.title-container {
  position: relative;

  .title {
    font-size: 26px;
    color: black;
    margin: 0px auto 40px auto;
    text-align: center;
    font-weight: bold;
  }
}

.show-pwd {
  position: absolute;
  right: 10px;
  top: 7px;
  font-size: 16px;
  color: $dark_gray;
  cursor: pointer;
  user-select: none;
}
</style>



<style scoped>
/* 修改验证器样式 */
::v-deep .el-form-item.is-error .el-input__inner {
  border-color: #bfd0d9;
}
::v-deep .el-form-item.is-error .el-input__validateIcon {
  color: #bac2c7;
}
::v-deep .el-form-item__error {
  color: #e6a23c;
}
</style>
