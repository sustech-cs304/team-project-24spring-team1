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

          <el-form-item label="enter your name" prop="name">
            <el-col :span="10">
              <el-input v-model="ruleForm.name" />
            </el-col>
          </el-form-item>

          <el-form-item label="sustech_id" prop="sustech_id">
            <el-col :span="10">
              <el-input
                v-model="ruleForm.sustech_id"
                placeholder="enter your sustech id"
              />
            </el-col>
          </el-form-item>
<!--            <el-button-->
<!--              :loading="codeLoading"-->
<!--              :disabled="isDisable"-->
<!--              size="small"-->
<!--              round-->
<!--              @click="sendMsg"-->
<!--            >send verification code</el-button>-->
<!--            <span class="status">{{ statusMsg }}</span>-->


<!--          <el-form-item label="verification" prop="code">-->
<!--            <el-col :span="10">-->
<!--              <el-input-->
<!--                v-model="ruleForm.code"-->
<!--                maxlength="6"-->
<!--                placeholder="verification code sent to your email please check"-->
<!--              />-->
<!--            </el-col>-->
<!--          </el-form-item>-->

          <el-form-item label="pwd" prop="pwd">
            <el-col :span="10">
              <el-input v-model="ruleForm.pwd" type="password"
              placeholder="enter your password"/>
            </el-col>
          </el-form-item>

          <el-form-item label="confirm pwd" prop="cpwd">
            <el-col :span="10">
              <el-input v-model="ruleForm.cpwd" type="password"
              placeholder="enter your password again"/>
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
  data() {
    return {
      statusMsg: '',
      error: '',
      isDisable: false,
      codeLoading: false,
      ruleForm: {
        sustech_id: '', // email: '',
        name: '',
        // code: '',
        pwd: '',
        cpwd: ''
      },
      rules: {
        name: [{required:true,trigger:'blur',message: 'enter name'}],
        sustech_id: [
          {required: true,
          trigger: 'blur',
          message: 'enter your sustech_id' }
        ],
        // code:[{required:false, trigger:'blur'}],
        pwd: [{
          required: true,
          message: 'set password',
          trigger: 'blur'
        }],
          //{ pattern: /^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,20}$/, message: 'contain both letters and digits, length 8-20.' }],
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

    register: function() {
      this.$refs['ruleForm'].validate((valid) => {
        if (valid) {
          const apiUrl = `https://backend.sustech.me/api/auth/register`;
          const userData = {
            sustech_id: parseInt(this.ruleForm.sustech_id),
            name: this.ruleForm.name,
            password: this.ruleForm.pwd //encrypt(this.ruleForm.pwd)
          };

          axios.post(apiUrl, userData)  // register(user).then(res => {
              .then(response => {
                const userData = response.data;
                this.account_id = userData.account_id;
                this.showSuccessMessage(`注册成功 ${this.account_id}`);

                this.$message({
                  showClose: true,
                  message: 'successfully registered, linking to login page...',
                  type: 'success'
                });
                setTimeout(() => {
                  this.$router.push('/')
                }, 1000);

              })
              .catch(error => {
                console.error("data: ", userData);
                console.error('failed to create an account：', error);
                this.showFailMessage("注册失败");
              });
        }
      })
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



<style lang="scss" scoped>
$bg: #dc8a8a;
$cursor:white;
$bgImage: url('@/adminform/assets/sustech2.png'); // 替换为你的背景图片路径

body {
  min-height: 100vh;
  width: 100%;
  background-color: $bg;
  background-image: $bgImage;
  background-size: cover; // 使背景图片覆盖整个屏幕
  background-position: center; // 背景图片居中
  background-repeat: no-repeat; // 防止背景图片重复
  opacity: 0.8; // 设置透明度
  overflow: hidden;

  .header {
    border-bottom: 2px solid rgb(6, 7, 7);
    min-width: 980px;
    color: black;

    header {
      margin: 0 auto;
      padding: 10px 0;
      width: 980px;

      .login {
        float: right;
      }

      .bold {
        font-style: normal;
        color: black;
      }
    }
  }

  > section {
    display: flex;
    justify-content: center; // 水平居中
    align-items: center; // 垂直居中
    height: 100vh; // 确保容器有足够的高度来居中内容
    padding-top: 30px;
    box-sizing: border-box;
    width: 980px;
  }

  .status {
    font-size: 12px;
    margin-left: 10%;
    color: #e6a23c;
  }

  .error {
    color: #e13030;
  }

  .tips {
    float: right;
    font-size: 14px;
    color: #ce1e1e;
    margin-bottom: 20px;

    span {
      &:first-of-type {
        margin-right: 16px;
      }
    }
  }
}

.register-container {
  position: relative;
  background: rgba(255, 255, 255, 0.9); // 设置表单容器的背景颜色和透明度
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);

  .el-input {
    display: inline-block;
    height: 100%;
    width: 95%;

    input {
      background: rgba(0, 0, 0, 0.1);
      border-radius: 5px;
      border: 1px solid rgba(150, 127, 127, 0.1);
      -webkit-appearance: none;
      padding: 12px 5px 12px 15px;
      color: lightgray;
      height: 50px;
      caret-color: lightgray;

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
      color: black;
    }
  }
}
</style>

<style lang="scss" scoped>
$bg: #fafafa;

body {
  min-height: 100%;
  width: 100%;
  background-color: $bg;
  overflow: hidden;

  .header {
    border-bottom: 2px solid rgb(6, 7, 7);
    min-width: 980px;
    color: black;

    header {
      margin: 0 auto;
      padding: 10px 0;
      width: 980px;

      .login {
        float: right;
      }

      .bold {
        font-style: normal;
        color: black;
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
      color: #e13030;
    }
  }

  .tips {
    float: right;
    font-size: 14px;
    color: #ce1e1e;
    margin-bottom: 20px;

    span {
      &:first-of-type {
        margin-right: 16px;
      }
    }
  }
}
</style>


<!--<style scoped>-->
<!--/* 修改验证器样式 */-->
<!--::v-deep .el-form-item.is-error .el-input__inner {-->
<!--  border-color: #889aa4;-->
<!--}-->
<!--::v-deep .el-form-item.is-error .el-input__validateIcon {-->
<!--  color: #c2ddec;-->
<!--}-->
<!--::v-deep .el-form-item__error {-->
<!--  color: #eaa43a;-->
<!--}-->
<!--</style>-->
