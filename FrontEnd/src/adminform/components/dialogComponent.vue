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

        <el-form-item label="roomName" prop="roomName">
          <el-input v-model="formInfo.activityName" placeholder="room name" clearable />
        </el-form-item>

        <el-form-item label="department" prop="department" required>
          <el-input placeholder="department" v-model="formInfo.department" clearable />
        </el-form-item>

        <el-form-item label="roomType" prop="roomType">
          <el-radio v-model="formInfo.roomType" label="small">small</el-radio>
          <el-radio v-model="formInfo.roomType" label="medium">medium</el-radio>
          <el-radio v-model="formInfo.roomType" label="big">big</el-radio>
        </el-form-item>

        <el-form-item label="date" prop="date" label-width="150px">
          <el-date-picker
              v-model="formInfo.date"
              type="date"
              placeholder="Pick a date"
              value-format="yyyy-MM-dd"
              style="width: 250px"
              :picker-options="dateOptions"
              clearable
          />
        </el-form-item>

        <el-form-item prop="startTime">
        <el-time-select
            style="width:150px;"
            :clearable="false"
            placeholder="startTime"
            v-model="formInfo.startTime"
            :picker-options="{
              start: '00:00',
              step: '00:01',
              end:formInfo.startTime?formInfo.startTime:'23:59',
              maxTime: formInfo.endTime
          }">
        </el-time-select>
        </el-form-item>
        <span class="">-</span>

        <el-form-item prop="endTime">
        <el-time-select
            style="width:150px;"
            placeholder="endTime"
            :clearable="false"
            v-model="formInfo.endTime"
            :picker-options="{
              start: formInfo.endTime?param.endTime:'00:00',
              step: '00:01',
              end:'24:00',
              minTime: formInfo.startTime
            }">
        </el-time-select>
        </el-form-item>

        <el-form-item label="loc1" prop="loc1">
          <el-select
              v-model="formInfo.loc1"
              placeholder="department"
              clearable
          >
            <el-option label="--" value="none" />
            <el-option label="Teaching Building No.1" value="Teaching-Building-No.1" />
            <el-option label="Lecture Hall" value="Lecture-Hall" />
            <el-option label="Research Building Lecture Hall" value="Research Building Lecture Hall" />
            <el-option label="Library Conference Hall" value="Library Conference Hall" />
            <el-option label="south Building" value="South Building" />
          </el-select>
        </el-form-item>

        <el-form-item label="loc2" prop="loc2">
          <el-input v-model="formInfo.loc2" placeholder="detail location" clearable />
        </el-form-item>

        <el-form-item label="duration" prop="duration">
          <el-input v-model="formInfo.duration" placeholder="max duration" clearable />
        </el-form-item>

        <el-form-item label="上传图片" prop="images">
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

        <el-form-item>
<!--          <el-button @click="getFile" style="margin-top: 10px">-->
<!--            <i class="el-icon-upload"></i>&nbsp;{{$i18n.t('CLICK_UPLOAD')}}-->
<!--          </el-button>-->
<!--          <input type="file" ref="file" style="display: none;" v-on:change="handleFileUpload($event)">-->
          <el-upload
              action="https://jsonplaceholder.typicode.com/posts/"
              :http-request="onUpload"
          >
            <el-button size="small" type="primary">点击上传</el-button>
          </el-upload>
        </el-form-item>

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
//import dateP from "@/components/dateP.vue";
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
    /*ifE:{
      type: Boolean,
      default: false
    },*/
  },

  data() {
    return {
      showDialog: false,
      formInfo: JSON.parse(JSON.stringify(this.itemInfo)),
      param:{
        startInfo:"08:00",
        endInfo:"23:00"
      },
      formInfoRules:{
        AcitivityName: [
          { required:true,message: "please enter activity name", trigger: "blur" },
        ],
        roomType: [{required:true}],
        loc1:[
          { required:true ,message:"choose end time",trigger:blur},
          { pattern: /^(?!none$)/, message: 'Please select a valid option', trigger: 'change' }
        ],
        startTime: [{required:true,message:"choose start time",trigger:blur},
          {}],
        endTime: [
          { required:true,message:"choose end time",trigger:blur},
          {
            //min:tableItem.startTime,
            message: "end time no earlier than startTime",
            trigger:"blur"
          }
        ],
        date:[
          { required:true,message:"choose end time",trigger:blur},
          {
            //min: "2023-10-11",
            message: "please choose a date no earlier",
            trigger:"blur"
          }
        ],
        department: [
          { required: true, message: "请输入", trigger: "blur" },
          {
            pattern: /^[A-Za-z]+$/,
            message: "letters",
            trigger: "blur",
          }
        ],
        loc2:[{required:false,message:"in",trigger:"blur"},
          {
            pattern: /^\d+[A-E]+$/,
            message: "layer+room(A-E)",
            trigger: "blur"
          }],
        duration:[{required:true,message:"input",trigger:"blur"},
          {
            pattern:/^\d+$/,
            message: "numbers",
            trigger: "blur"
          }],
        description:[{required:false, message:"describe the activity",trigger:"blur"}],
        tickets: [{required:true, trigger:"blur"}],
        deadline: [{required:true, trigger:"blur"}],
      },
      timeInfo:{
        startTimeInfo: '08:00',
        endTimeInfo: '08:00',
      },
      dateOptions: {
        disabledDate(time) {
          return time.getTime() < Date.now() - 8.64e7;
        }
      },
    };
  },
  mounted() {

  },
  methods: {
    // 上传文件
    onUpload (file) {
      let formData = new FormData()
      formData.append('file',file.file)
      postUpload(formData).then((res) => {
        console.log(res)
        this.$message.success(this.$t('UPLOAD_SUCCESS'))
      }).catch((e) => {
        this.$message.error(e.message)
      })
    },

    // 打开文件
    getFile () {
      this.$refs.file.click()
    },
    handleFileUpload(event){
      // 阻止发生默认行为
      event.preventDefault();
      let formData = new FormData()
      let file = this.$refs.file.files[0]
      formData.append('file',file)
      // console.log(formData.get('file'))
      this.onUpload(formData)
    },

    submitForm(formName) {
      const that = this;
      //const params = Object.assign(this.formInfo, {});
      that.$refs[formName].validate((valid) => {
        if (valid) {
          that.$message({
            message: "success! ",
            type: "success",
          });
          //console.log(this.dialogTitle) console.log(that.formInfo)
          const sentData={
            activityName:this.formInfo.roomName,
            department:this.formInfo.department,
            roomType:this.formInfo.roomType,
            loc1:this.formInfo.loc1,
            loc2:this.formInfo.loc2,
            date:this.formInfo.date,
            startTime:this.formInfo.startTime,
            endTime:this.formInfo.endTime,
            duration:this.formInfo.duration,
          }
          const requestData={
            name:this.formInfo.roomName,
            kind:this.formInfo.roomType,
            description: this.formInfo.description,
            venue_id:this.formInfo.loc1,
            // date:this.formInfo.date,
            start_at:this.formInfo.startTime,
            end_at:this.formInfo.endTime,
            tickets: this.formInfo.tickets,
            registration_deadline: this.formInfo.deadline,
          }

          const apiUrl = `https://backend.sustech.me/api/event`;
          axios.post(apiUrl,requestData)
              .then(response => {
                const actiData = response.data;
                this.activityId = actiData.id;
              })
              .catch(error => {
                console.error('failed to publish event：', error);
              })

          this.$emit("pushDialogData",sentData);
          that.closeDialog(1);
        } else {
          return false;
        }
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
      //const params = Object.assign(this.formInfo, {});
      that.$refs[formName].validate((valid) => {
        if (valid) {
          that.$message({
            message: "success！",
            type: "success",
          });
          //console.log(this.dialogTitle) console.log(that.formInfo)
          const sentData={
            roomName:this.formInfo.roomName,
            department:this.formInfo.department,
            roomType:this.formInfo.roomType,
            loc1:this.formInfo.loc1,
            loc2:this.formInfo.loc2,
            date:this.formInfo.date,
            startTime:this.formInfo.startTime,
            endTime:this.formInfo.endTime,
            duration:this.formInfo.duration,
          }
          this.$emit('editDialog',sentData)
          this.$parent.tableData[this.$parent.editrowNum]=sentData;
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