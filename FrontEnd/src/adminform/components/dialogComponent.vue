<template>
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

<!--        <el-form-item label="date" prop="date" label-width="150px">-->
<!--          <el-date-picker-->
<!--              v-model="formInfo.date"-->
<!--              type="date"-->
<!--              placeholder="Pick a date"-->
<!--              value-format="yyyy-MM-dd"-->
<!--              style="width: 250px"-->
<!--              :picker-options="dateOptions"-->
<!--              clearable-->
<!--          />-->
<!--        </el-form-item>-->

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
<!--            <el-option label="Lecture Hall" value=2 />-->
<!--            <el-option label="Research Building Lecture Hall" value=3 />-->
<!--            <el-option label="Library Conference Hall" value=4 />-->
<!--            <el-option label="south Building" value=5 />-->
<!--          </el-select>-->
<!--        </el-form-item>-->

        <el-form-item label="venue_id" prop="venue_id">
          <el-input v-model="formInfo.venue_id" placeholder="venue_id" clearable :pattern="'^[0-9]+$'" />
        </el-form-item>

        <el-form-item label="tickets" prop="tickets">
          <el-input v-model="formInfo.tickets" placeholder="tickets available" clearable :pattern="'^[0-9]+$'" />
        </el-form-item>

        <el-form-item label="UPLOAD IMAGE" prop="images">
          <el-upload
              action=""
              :limit="3"
              list-type="picture-card"
              :on-exceed="handleExceed"
              :before-upload="beforeUpload"
              :on-remove="handleRemove"
              :file-list="fileList"
          >
            <i class="el-icon-plus avatar-uploader-icon"></i>
          </el-upload>
        </el-form-item>

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

<!--        <el-form-item>-->
<!--          <el-upload-->
<!--              action="https://jsonplaceholder.typicode.com/posts/"-->
<!--              :http-request="onUpload"-->
<!--          >-->
<!--            <el-button size="small" type="primary">CLICK TO UPLOAD</el-button>-->
<!--          </el-upload>-->
<!--        </el-form-item>-->

        <el-form-item style="text-align: right;">
          <el-button type="primary" @click="submitForm('formInfo')" size="medium">confirm publish</el-button>
          <el-button type="primary" @click="submitFormEdit('formInfo')" size="medium">submit change</el-button>
          <el-button type="primary" @click="closeDialog(0)">cancel</el-button>
          <el-button type="primary" @click="clearEntry">clear</el-button>
        </el-form-item>

      </el-form>
    </el-dialog>
  </transition>
</template>


<script>
import axios from "axios";

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
      formInfo: JSON.parse(JSON.stringify(this.itemInfo)),
      param:{
        startInfo:"08:00",
        endInfo:"23:00"
      },
      activityId: 0,
      formInfoRules:{
        AcitivityName: [
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
            venue_id:parseInt(this.formInfo.venue_id, 10), // 将字符串转换为i32类型 10进制
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
                this.activityId = response.data.id;
                that.$message({
                  message: "发布成功",
                  type: "success",
                });
              })
              .catch(error => {
                console.error("data:", eventData);
                console.error('failed to publish event：', error);
                this.showFailMessage(`发布活动失败`);
              })
          this.$emit("pushDialogData",eventData);
          that.closeDialog(1);
        } else {
          return false;
        }

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
      this.fileList = this.fileList.filter(item => item.uid !== file.uid);
    },
    handleExceed() {
      this.msgError("最多只能传3张照片");
    },
    beforeUpload(file) {
      const isJPG = file.type === "image/jpeg" || file.type == "image/png";
      const isLt2M = file.size / 1024 / 1024 < 2;
      if (!isJPG) {
        this.$message.error("上传头像图片只能是 JPG 或 PNG 格式!");
        return;
      }
      if (!isLt2M) {
        this.$message.error("上传头像图片大小不能超过 2MB!");
        return;
      }
      const fileData = new FormData();
      fileData.append("avatar", file);
      //upload为上传的接口
      upload(fileData).then(res => {
        this.imgUrl = res.imgUrl;
        //对返回的图片地址进行回显
        this.$set(this.form, "avatar", this.imgUrl);
      });
      //阻止传到本地浏览器
      return false;
    },

    submitFormEdit(formName) {
      const that = this;
      that.$refs[formName].validate((valid) => {
        if (valid) {
          that.$message({
            message: "success！",
            type: "success",
          });
          //console.log(this.dialogTitle) console.log(that.formInfo)
          const eventData={
            name:this.formInfo.activityName,
            kind:this.formInfo.kind,
            description: this.formInfo.description,
            venue_id:this.formInfo.venue_id,
            start_at:this.formInfo.startTime,
            end_at:this.formInfo.endTime, //"2021-05-21T14:00:00",
            tickets: this.formInfo.tickets, //100 | null,
            registration_deadline: this.formInfo.deadline, //"2021-05-21T11:00:00" | null
          }
          this.$emit('editDialog',eventData)
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