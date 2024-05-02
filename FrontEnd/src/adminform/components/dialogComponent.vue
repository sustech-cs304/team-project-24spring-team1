<template>
  <transition name="dialog-fade">
    <el-dialog
        v-if="showDialog"
        :title="dialogTitle"
        class="dialog-component"
        :visible.sync="showDialog"
        width="500px"
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
          <el-input v-model="formInfo.roomName" placeholder="room name" clearable />
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
            placeholder="起始时间"
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
            placeholder="结束时间"
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

        <el-form-item style="text-align: right;">
          <el-button type="primary" @click="submitForm('formInfo')" size="medium">确定新建</el-button>
          <el-button type="primary" @click="submitFormEdit('formInfo')" size="medium">提交修改</el-button>
          <el-button type="primary" @click="closeDialog(0)">取消</el-button>
          <el-button type="primary" @click="clearEntry">清空</el-button>
        </el-form-item>
77
      </el-form>
    </el-dialog>
  </transition>
</template>


<script>
//import dateP from "@/components/dateP.vue";
export default {
  name: "DialogComponent",
  //components:{dateP},
  props: {
    dialogTitle: {
      type: String,
      default: "添加",
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
        roomName: [
          { required:true,message: "请输入用户名", trigger: "blur" },
          {
            pattern:/^[A-Za-z]+\d+$/,
            message: "字母（大小写均可）+数字",
            trigger: "blur",
          },
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
      /*endtimepickerop:{
        disabledDate: (time) => {
          return time.getTime() > this.formInfo.startTime;
        }
      }*/
    };
  },

  methods: {
    submitForm(formName) {
      const that = this;
      //const params = Object.assign(this.formInfo, {});
      that.$refs[formName].validate((valid) => {
        if (valid) {
          that.$message({
            message: "操作成功！",
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
          // if(this.$props) {
          //   this.$emit('editDialog',sentData)
          // } else {
          this.$emit("pushDialogData",sentData);
          //与that不同，this是空白that是无]undefined
          //else that.$emit("editData",this.$refs[formName])
          that.closeDialog(1);
          /*const dialogData=this.$refs["dialogComponent"].$data;
          this.tableData.push(dialogData);*/
        } else {
          return false;
        }
      });
    },

    submitFormEdit(formName) {
      const that = this;
      //const params = Object.assign(this.formInfo, {});
      that.$refs[formName].validate((valid) => {
        if (valid) {
          that.$message({
            message: "操作成功！",
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