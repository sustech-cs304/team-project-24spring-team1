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

      <el-button :loading="loading" type="primary" style="width:100%;margin-bottom:30px;" @click.native.prevent="handleLogin">LOGIN</el-button>

      <p class="tips">
        <a href="#/register" type="primary">no account? register now.</a>
      </p>

    </el-form>
  </div>
</template>

<script>
import axios from 'axios';
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
    async handleLogin() {
      const apiUrl = `https://backend.sustech.me/api/auth/login`;
      const requestData = {
        sustech_id: this.userName,
        password: this.password
      };
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
          localStorage.setItem('jwt_token', userData.token);
          if (userData.token && userData.account_id) {
            // clearInterval(pollInterval);
            this.showMessage("登录成功");
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
          this.showMessage("登录失败! ");
        }
        },

    showMessage(message) {
      this.successMessage = message;
      // 显示弹窗
      this.$message({
        message: this.successMessage,
        type: 'success'
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
    border: 1px solid rgba(255, 255, 255, 0.1);
    background: rgba(0, 0, 0, 0.1);
    border-radius: 5px;
    color: #454545;
  }
}
</style>

<style lang="scss" scoped>
$bg:#2d3a4b;
$dark_gray:#889aa4;
$light_gray:#eee;

body {
  min-height: 100%;
  width: 100%;
  background-color: $bg;
  overflow: hidden;

  .login-form {
    position: relative;
    width: 520px;
    max-width: 100%;
    padding: 160px 35px 0;
    margin: 0 auto;
    overflow: hidden;
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
      color: $light_gray;
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
}
</style>



<style scoped>
/* 修改验证器样式 */
::v-deep .el-form-item.is-error .el-input__inner {
  border-color: #889aa4;
}
::v-deep .el-form-item.is-error .el-input__validateIcon {
  color: #889aa4;
}
::v-deep .el-form-item__error {
  color: #e6a23c;
}
</style>
