<template>
  <!-- 编辑活动的窗口 -->
  <transition name="dialog-fade">
    <el-dialog
        v-if="showDialog"
        :title="dialogTitle"
        class="dialog-component"
        :visible.sync="showDialog"
        width="600px"
        @close="closeDialog(0)"
    >

      <el-form
          ref="formInfo"
          :model="formInfo"
          :rules="formInfoRules"
          class="demo-form-inline"
          label-width="100px"
          :inline="true" >

        <el-form-item label="activityName" prop="activityName">
          <el-input v-model="formInfo.activityName" placeholder="activity name" clearable/>
        </el-form-item>

        <div style="display:flex;flex-wrap:wrap;">
        <el-form-item label="kind" prop="kind">
          <el-radio v-model="formInfo.kind" label="show">show</el-radio>
          <el-radio v-model="formInfo.kind" label="lecture">lecture</el-radio>
          <el-radio v-model="formInfo.kind" label="competition">competition</el-radio>
          <el-radio v-model="formInfo.kind" label="other">other</el-radio>
        </el-form-item>
        </div>

        <el-form-item prop="startTime">
          <el-date-picker
              v-model="formInfo.startTime"
              :clearable="true"
              type="datetime"
              placeholder="startTime">
          </el-date-picker>
<!--          picker-options="{&ndash;&gt;-->
          <!--              start: '00:00',-->
          <!--              step: '00:01',-->
          <!--              end:formInfo.startTime?formInfo.startTime:'23:59',-->
          <!--              maxTime: formInfo.endTime-->
          <!--          }">-->
        </el-form-item>
        <span class="">-</span>

        <el-form-item prop="endTime">
        <el-date-picker
            style="width:200px;"
            placeholder="endTime"
            type="datetime"
            :clearable="false"
            v-model="formInfo.endTime"
            :picker-options="{
              // start: formInfo.endTime?param.endTime:'00:00',
              // step: '00:01',
              // end:'24:00',
              // minTime: formInfo.startTime
            }">
        </el-date-picker>
        </el-form-item>

<!--        <el-form-item label="venue_id" prop="venue_id">-->
<!--          <el-select-->
<!--              v-model="formInfo.venue_id"-->
<!--              placeholder="venue_id"-->
<!--              clearable-->
<!--          >-->
<!--            <el-option label="&#45;&#45;" value="none" />-->
<!--            <el-option label="Teaching Building No.1" value=1 />-->
<!--          </el-select>-->
<!--        </el-form-item>-->

        <el-form-item label="venue_id" prop="venue_id">
          <el-input v-model="formInfo.venue_id" placeholder="venue_id" clearable :pattern="'^[0-9]+$'" />
        </el-form-item>

        <el-form-item label="latitude" prop="latitude">
          <el-input v-model="formInfo.latitude" placeholder="latitude" clearable />
        </el-form-item>

        <el-form-item label="longtitude" prop="longtitude">
          <el-input v-model="formInfo.longtitude" placeholder="longtitude" clearable />
        </el-form-item>

        <el-form-item label="tickets" prop="tickets">
          <el-input v-model="formInfo.tickets" placeholder="tickets available" clearable :pattern="'^[0-9]+$'" />
        </el-form-item>

<!--        <el-form-item label="upload image" prop="cover">-->
<!--          <el-upload-->
<!--              ref="upload"-->
<!--              class="upload-demo"-->
<!--              action="https://backend.sustech.me/upload"-->
<!--              list-type="picture-card"-->
<!--              :before-upload = "beforeUpload"-->
<!--              :accept="'.jpg,.jpeg,.png,.gif'"-->
<!--              :on-success="handleSuccess"-->
<!--              :on-remove = "handleRemove"-->
<!--              :limit = "1"-->
<!--              name="file"-->
<!--          >-->
<!--            <i class="el-icon-plus avatar-uploader-icon"></i>-->
<!--          </el-upload>-->
<!--        </el-form-item>-->

        <div class="row">
          <div class="col-md-12">
            <input type="file" @change="handleFileUpload" style="display: none"
                   ref="fileInput" accept=".jpg, .jpeg, .png, .gif">
            <img class="avatar" :src="imageUrl" alt="User Avatar" style="width: 200px;
                    height: 200px;" @click="openFileInput"/>
          </div>
        </div>

        <el-form-item prop="deadline">
          <el-date-picker
              v-model="formInfo.deadline"
              :clearable="true"
              type="datetime"
              placeholder="deadline">
          </el-date-picker>
        </el-form-item>

        <el-form-item label="description" prop="description">
          <el-input v-model="formInfo.description" placeholder="description of your activity"
                    clearable :pattern="'^[0-9]+$'" />
        </el-form-item>

        <el-form-item style="display: flex; justify-content: space-between;">
          <el-button type="primary" @click="submitForm('formInfo')"
                     size="medium"  style="margin: 0 10px;">PUBLISH</el-button>
<!--          <el-button type="primary" @click="submitFormEdit('formInfo')" size="medium">submit change</el-button>-->
          <el-button type="primary" @click="closeDialog(0)" style="margin: 0 10px;">CANCEL</el-button>
          <el-button type="primary" @click="clearEntry"
                     style="margin: 0 10px;">CLEAR</el-button>
        </el-form-item>

      </el-form>
    </el-dialog>
  </transition>
</template>


<script>
import axios from "axios";
import {float} from "mockjs/src/mock/random/basic";

export default {
  name: "DialogComponent",
  //components:{dateP},
  props: {
    dialogTitle: {
      type: String,
      default: "publish event",
    },
    itemInfo: {
      type: Object,
      default: function () {
        return {};
      },
    },
  },

  data() {
    return {
      showDialog: false,
      fileToUpload: null,
      imageUrl: '',
      formInfo: JSON.parse(JSON.stringify(this.itemInfo)),
      formInfoRules:{
        acitivityName: [
          { required:true,message: "please enter activity name", trigger: "blur" },
        ],
        kind: [{required:true}],
        venue_id:[{ required:true ,message:"venue",trigger:blur}],
        startTime: [{required:true,message:"choose start time",trigger:blur}],
        endTime: [{ required:true,message:"choose end time",trigger:blur},],
        description: [{required: false, message: "enter description", trigger: "blur" },],
        tickets:[{required:true,message:"tickets available",trigger:"blur"},
          {
            pattern: /^\d+$/,
            message: "number of tickets, no less than 0",
            trigger: "blur"
          }],
        deadline: [{required:true, trigger:"blur"}],
      },
      dateOptions: {
        disabledDate(time) {
          return time.getTime() < Date.now() - 8.64e7;
        }
      },
    };
  },

  methods: {
    submitForm(formName) {
      const that = this;
      // this.uploadFile();
      console.log("上传图片之后： ",this.formInfo.cover);
      //const params = Object.assign(this.formInfo, {});
      that.$refs[formName].validate((valid) => {
        if (valid) {
          //console.log(this.dialogTitle) console.log(that.formInfo)
          const start_iso = new Date(this.formInfo.startTime).toISOString();
          const end_iso = new Date(this.formInfo.endTime).toISOString();
          const deadline_iso = new Date(this.formInfo.deadline).toISOString();
          const location = [parseFloat(this.formInfo.latitude), parseFloat(this.formInfo.longtitude)];
          const eventData={
            name:this.formInfo.activityName,
            kind:this.formInfo.kind,
            description: this.formInfo.description,
            cover: this.formInfo.cover,
            venue_id:parseInt(this.formInfo.venue_id, 10), // 将字符串转换为i32类型 10进制
            location: location,
            start_at:start_iso.substring(0, start_iso.length - 5),
            end_at:end_iso.substring(0, end_iso.length - 5), //"2021-05-21T14:00:00",
            tickets: parseInt(this.formInfo.tickets), //100 | null,
            registration_deadline: deadline_iso.substring(0, start_iso.length - 5), //"2021-05-21T11:00:00" | null
          }
          // this.imageUrl = `https://backend.sustech.me/uploads/${userData.avatar}.webp`;
          this.token = localStorage.getItem('token');
          if (!this.token) {
            console.log("Token not found.");
            return;
          }

          const apiUrl = `https://backend.sustech.me/api/event`;
          axios.post(apiUrl, eventData,
              {timeout: 3000,headers: {
                  Authorization: `Bearer ${this.token}`
                }
              })
              .then(response => {
                that.$message({
                  message: "发布成功",
                  type: "success",
                });
                this.$emit("pushDialogData",eventData);
                console.log("成功发布： ",eventData);
                eventData.id = response.data.id;
              })
              .catch(error => {
                console.error('failed to publish event：', error);
                this.showFailMessage(`发布活动失败`);
              })
          that.closeDialog(1);
        } else {
          return false;
        }
      });
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
      this.uploadFile();
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
            const uuid = response.data.split('/').pop().replace('.webp', '');
            this.formInfo.cover = uuid;
            this.showSuccessMessage(uuid);
          })
          .catch(error => {
            this.showFailMessage("图片上传失败");
            console.error('Error uploading file:', error);
          });
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
    handleRemove(file) {
      // this.fileList = this.fileList.filter(item => item.uid !== file.uid);
      this.$message.info('文件已删除');
      this.imageUrl = ''; // 清除已删除的图片URL
    },
    beforeUpload(file) {
      // 在上传前进行一些检查或预处理，例如文件类型和大小
      const isJPG = file.type === 'image/jpeg';
      const isPNG = file.type === 'image/png';
      const isGIF = file.type === 'image/gif';
      const isLt2M = file.size / 1024 / 1024 < 2;

      if (!isJPG && !isPNG && !isGIF) {
        this.$message.error('上传图片只能是 JPG, PNG 或 GIF 格式!');
        return false;
      }
      if (!isLt2M) {
        this.$message.error('上传图片大小不能超过 2MB!');
        return false;
      }
      // this.$message.success('before handling succeded');
      return true;
      // const fileData = new FormData();
      // fileData.append("avatar", file);
      // //upload为上传的接口
      // upload(fileData).then(res => {
      //   this.imgUrl = res.imgUrl;
      //   //对返回的图片地址进行回显
      //   this.$set(this.form, "avatar", this.imgUrl);
      // });
      // //阻止传到本地浏览器
      // return false;
    },
    handleSuccess(response, file, fileList) {
      // 处理上传成功的响应，例如获取上传后的图片URL
      this.$message.success('上传成功');
      this.uploadedImageUrl = response; // 假设服务器返回的响应是图片的URL
      console.log('Uploaded file URL:', this.uploadedImageUrl);
    },
    handleUpload(){
      const imgUrl = "https://backend.sustech.me/upload";
      const formData = new FormData();
      formData.append("file", this.file); // file 是一个 File 对象，代表要上传的文件

      axios.post(imgUrl, formData, {
        headers: {
          "Content-Type": "multipart/form-data"
        }
      })
          .then(response => {
            const url = response.data;
            this.showSuccessMessage("上传图片成功");
            // console.log("上传成功，图片 URL:", response.data);
          })
          .catch(error => {
            this.showFailMessage("上传图片失败");
            console.error('上传图片失败：', error);
          });
    },

    submitFormEdit(formName) {
      const that = this;
      //const params = Object.assign(this.formInfo, {});
      that.$refs[formName].validate((valid) => {
        if (valid) {
          //console.log(this.dialogTitle) console.log(that.formInfo)
          const start_iso = new Date(this.formInfo.startTime).toISOString();
          const end_iso = new Date(this.formInfo.endTime).toISOString();
          const deadline_iso = new Date(this.formInfo.deadline).toISOString();
          const eventData={
            name:this.formInfo.activityName,
            kind:this.formInfo.kind,
            description: this.formInfo.description,
            cover:this.formInfo.cover,
            venue_id:parseInt(this.formInfo.venue_id, 10), // 将字符串转换为i32类型 10进制
            start_at:start_iso.substring(0, start_iso.length - 5),
            end_at:end_iso.substring(0, end_iso.length - 5), //"2021-05-21T14:00:00",
            latitude:this.formInfo.latitude,
            longtitude:this.formInfo.longtitude,
            tickets: parseInt(this.formInfo.tickets), //100 | null,
            registration_deadline: deadline_iso.substring(0, start_iso.length - 5), //"2021-05-21T11:00:00" | null
          }
          // this.imageUrl = `https://backend.sustech.me/uploads/${userData.avatar}.webp`;
          this.token = localStorage.getItem('token');
          if (!this.token) {
            console.log("Token not found.");
            return;
          }

          const apiUrl = `https://backend.sustech.me/api/event`;
          axios.post(apiUrl, eventData, {timeout: 3000,headers: {
                  Authorization: `Bearer ${this.token}`
                }
              })
              .then(response => {
                that.$message({
                  message: "修改活动成功",
                  type: "success",
                });
                this.$emit('editDialog',eventData)
                eventData.id = response.data.id;
              })
              .catch(error => {
                console.error('failed to publish event：', error);
                this.showFailMessage(`修改活动失败`);
              })

          this.$parent.tableData[this.$parent.editrowNum]=eventData;
          that.closeDialog(1);

        } else {
          return false;
        }
      });

    },

    closeDialog(flag) {
      this.showDialog = false;
      this.$emit("closeDialog", flag);
      this.$refs["formInfo"].resetFields();
    },
    clearEntry() {
      //this.formInfo="";
      this.$refs["formInfo"].resetFields();
    },

  },
};

</script>



<style scoped lang="scss">

.dialog-fade-enter-active {
  -webkit-animation: dialog-fade-in 0.3s;
  animation: dialog-fade-in 0.3s;
}
.dialog-fade-leave-active {
  -webkit-animation: dialog-fade-out 0.3s;
  animation: dialog-fade-out 0.3s;
}
@-ms-keyframes dialog-fade-in {
  0% {
    -ms-transform: translate3d(0, -20px, 0);
    transform: translate3d(0, -20px, 0);
    opacity: 0;
  }
  100% {
    -ms-transform: translate3d(0, 0, 0);
    transform: translate3d(0, 0, 0);
    opacity: 1;
  }
}
@keyframes dialog-fade-in {
  0% {
    -ms-transform: translate3d(0, -20px, 0);
    transform: translate3d(0, -20px, 0);
    opacity: 0;
  }
  100% {
    -ms-transform: translate3d(0, 0, 0);
    transform: translate3d(0, 0, 0);
    opacity: 1;
  }
}
@-ms-keyframes dialog-fade-out {
  0% {
    -ms-transform: translate3d(0, 0, 0);
    transform: translate3d(0, 0, 0);
    opacity: 1;
    /*@-webkit-*/
  }
  100% {
    -ms-transform: translate3d(0, -20px, 0);
    transform: translate3d(0, -20px, 0);
    opacity: 0;
  }
}
@keyframes dialog-fade-out {
  0% {
    -ms-transform: translate3d(0, 0, 0);
    transform: translate3d(0, 0, 0);
    opacity: 1;
  }
  100% {
    -ms-transform: translate3d(0, -20px, 0);
    transform: translate3d(0, -20px, 0);
    opacity: 0;
  }
}
</style>