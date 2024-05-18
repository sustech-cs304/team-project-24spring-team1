<template>
  <div class="register-container">
    <article class="header">
      <header>
        <el-avatar icon="el-icon-user-solid" shape="circle" />
        <span class="login">
          <em class="bold">already have an account?</em>
          <a href="/login">
            <el-button type="primary" size="mini">LOGIN</el-button>
          </a>
        </span>
      </header>
    </article>
    <section>
      <el-form
        ref="ruleForm"
        :model="ruleForm"
        :rules="rules"
        label-width="100px"
        autocomplete="off"
        :hide-required-asterisk="true"
        size="medium"
      >
        <div style="padding-top: 10px">
          <el-form-item label="email" prop="email">
            <el-col :span="10">
              <el-input
                v-model="ruleForm.email"
                placeholder="enter your email address and click to do verification"
              />
            </el-col>
            <el-button
              :loading="codeLoading"
              :disabled="isDisable"
              size="small"
              round
              @click="sendMsg"
            >send verification code</el-button>

            <span class="status">{{ statusMsg }}</span>
          </el-form-item>
          <el-form-item label="verification" prop="code">
            <el-col :span="10">
              <el-input
                v-model="ruleForm.code"
                maxlength="6"
                placeholder="verification code sent to your email please check"
              />
            </el-col>
          </el-form-item>
          <el-form-item label="pwd" prop="pwd">
            <el-col :span="10">
              <el-input v-model="ruleForm.pwd" type="password" />
            </el-col>
          </el-form-item>
          <el-form-item label="confirm pwd" prop="cpwd">
            <el-col :span="10">
              <el-input v-model="ruleForm.cpwd" type="password" />
            </el-col>
          </el-form-item>
          <el-form-item>
            <el-button
              type="primary"
              style="width: 40%"
              @click="register"
            >REGISTER</el-button>
          </el-form-item>
        </div>
      </el-form>
    </section>

    <div class="error">{{ error }}</div>
  </div>
</template>

<script>
import { getEmailCode, register } from '@/api/register'
import { encrypt } from '@/login/utils/rsaEncrypt'
import axios from 'axios';

export default {
  name: 'Register',
  data() {
    return {
      statusMsg: '',
      error: '',
      isDisable: false,
      codeLoading: false,
      ruleForm: {
        sustech_id: '',
        email: '',
        code: '',
        pwd: '',
        cpwd: ''
      },
      rules: {
        email: [{
          required: true,
          type: 'email',
          message: 'please enter your email',
          trigger: 'blur'
        }],
        code: [{
          required: true,
          type: 'string',
          message: 'please enter verification code',
          trigger: 'blur'
        }],
        pwd: [{
          required: true,
          message: 'set password',
          trigger: 'blur'
        }, { pattern: /^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,20}$/, message: 'contain both letters and digits, length 8-20.' }],
        cpwd: [{
          required: true,
          message: 'confirm password',
          trigger: 'blur'
        }, {
          validator: (rule, value, callback) => {
            if (value === '') {
              callback(new Error('please enter password again'))
            } else if (value !== this.ruleForm.pwd) {
              callback(new Error('NOT the same pwd'))
            } else {
              callback()
            }
          },
          trigger: 'blur'
        }]

      }
    }
  },
  mounted() {
    const apiUrl = `https://backend.sustech.me/api/auth/register`;
    const requestData = {
      sustech_id: this.sustech_id,
      name: this.userName,
      password: this.password
    };

    axios.post(apiUrl, requestData)
        .then(response => {
          const userData = response.data;
          this.token = userData.token;
          console.log('successfully created account：', response.data);
        })
        .catch(error => {
          console.error('failed to create an account：', error);
        });
  },

  methods: {
    sendMsg: function() {
      const self = this
      let emailPass
      let timerid
      if (timerid) {
        return false
      }
      self.statusMsg = ''
      this.$refs['ruleForm'].validateField('email', (valid) => {
        emailPass = valid
      })
      // 向后台API验证码发送
      if (!emailPass) {
        self.codeLoading = true
        self.statusMsg = 'sending validation code...'
        getEmailCode(self.ruleForm.email).then(res => {
          this.$message({
            showClose: true,
            message: 'code sent successfully，lasts for 5 minutes',
            type: 'success'
          })
          let count = 60
          self.ruleForm.code = ''
          self.codeLoading = false
          self.isDisable = true
          self.statusMsg = `verification code sent, resend after ${count--} seconds.`
          timerid = window.setInterval(function() {
            self.statusMsg = `verification code sent, resend after ${count--} seconds.`
            if (count <= 0) {
              window.clearInterval(timerid)
              self.isDisable = false
              self.statusMsg = ''
            }
          }, 1000)
        }).catch(err => {
          console.log(err.response)
          this.isDisable = false
          this.statusMsg = ''
          this.codeLoading = false
          console.log(err.response.data.message)
        })
      }
    },

    // 用户注册
    register: function() {
      this.$refs['ruleForm'].validate((valid) => {
        if (valid) {
          const user = {
            code: this.ruleForm.code,
            email: this.ruleForm.email,
            password: encrypt(this.ruleForm.pwd)
          }
          register(user).then(res => {
            console.log(res)
            this.$message({
              showClose: true,
              message: 'successfully registered, linking to login page...',
              type: 'success'
            })
            setTimeout(() => {
              this.$router.push('/')
            }, 2000)
          }).catch(err => {
            console.log(err.response.data.message)
          })
        }
      })
    }
  }

}
</script>

<style lang="scss">
/* 修复input 背景不协调 和光标变色 */
/* Detail see https://github.com/PanJiaChen/vue-element-admin/pull/927 */

$bg: #283443;
$light_gray: #fff;
$cursor: #fff;

@supports (-webkit-mask: none) and (not (cater-color: $cursor)) {
  .register-container .el-input input {
    color: $cursor;
  }
}

/* reset element-ui css */
.register-container {
  // display: flex;
  //justify-content: center; /* 水平居中 */
  //align-items: center;

  .el-input {
    display: inline-block;
    height: 100%;
    width: 95%;

    input {
      background: rgba(0, 0, 0, 0.1);
      border-radius: 5px;
      border: 1px solid rgba(255, 255, 255, 0.1);
      -webkit-appearance: none;
      padding: 12px 5px 12px 15px;
      color: $light_gray;
      height: 50px;
      caret-color: $cursor;

      &:-webkit-autofill {
        box-shadow: 0 0 0px 1000px $bg inset !important;
        -webkit-text-fill-color: $cursor !important;
      }
    }
  }

  .el-form-item {
    label {
      font-style: normal;
      font-size: 12px;
      color: $light_gray;
    }
  }
}
</style>


<style lang="scss" scoped>
$bg: #2d3a4b;
$dark_gray: #889aa4;
$light_gray: #eee;

body {
  min-height: 100%;
  width: 100%;
  background-color: $bg;
  overflow: hidden;

  .header {
    border-bottom: 2px solid rgb(235, 232, 232);
    min-width: 980px;
    color: #666;

    header {
      margin: 0 auto;
      padding: 10px 0;
      width: 980px;

      .login {
        float: right;
      }

      .bold {
        font-style: normal;
        color: $light_gray;
      }
    }
  }

  > section {
    margin: 0 auto 30px;
    padding-top: 30px;
    width: 980px;
    min-height: 300px;
    padding-right: 100px;
    box-sizing: border-box;

    .status {
      font-size: 12px;
      margin-left: 10%;
      color: #e6a23c;
    }

    .error {
      color: red;
    }
  }

  .tips {
    float: right;
    font-size: 14px;
    color: #fff;
    margin-bottom: 20px;

    span {
      &:first-of-type {
        margin-right: 16px;
      }
    }
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
