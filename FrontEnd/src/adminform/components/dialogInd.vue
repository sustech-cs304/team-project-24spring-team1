<template>
  <!-- 显示的大表格 -->
  <div class="dialog-demo">
    <card-img></card-img>
<!--       :style="{ backgroundImage:  'url(' + imgUrl + ')' }">-->
    <p class="title" style="text-align: center"> Activity Management </p>
    <el-row style="margin-top: 25px;margin-bottom: 10px;
    display: flex; justify-content: space-between;">
      <base-button type="primary" @click="addItem" size="medium">PUBLISH</base-button>
      <base-button type="primary" @click="deleteSelectedRows" size="medium" style="margin-left:40px;">
        delete selection</base-button>
      <base-button type="primary" size="medium" @click="deleteAllRows" style="margin-left:40px;">
        delete all </base-button>
    </el-row>

    <el-table
        v-loading="tableLoading"
        :data="tableData.slice((currentPage-1)*pageSize,currentPage*pageSize)"
        :page-size="5" stripe pagination
        class="storeTable"
        style="width: 100%; margin-top: 30px;"
        @selection-change="handleSelectionChange"
    >
      <!--"v-for="item in tableItem" :key="checked"-->

      <el-table-column label="select" width="100" type="selection"></el-table-column>
      <el-table-column label="id" prop="id" width="80"></el-table-column>
      <el-table-column label="activityName" prop="activityName" width="120"></el-table-column>
      <el-table-column label="kind" prop="kind" width="115"></el-table-column>
      <el-table-column label="venue_id" prop="venue_id" width="110"></el-table-column>
      <el-table-column label="startTime" prop="startTime" width="110"></el-table-column>
      <el-table-column label="endTime" prop="endTime" width="110"></el-table-column>
<!--      <el-table-column label="deadline" prop="duration" width="100"></el-table-column>-->
      <el-table-column label="description" prop="description" width="150"></el-table-column>

      <el-table-column label="operation" width="100">
        <template slot-scope="scope">
          <base-button type="primary" size=sm id="editBtn"
                     @click.native.prevent="editRow(scope.$index,scope.row)" round>EDIT</base-button>
          <base-button type="primary" size=sm id="deleteBtn" style="background-color: red"
                     @click.native.prevent="deleteRow(scope.$index,true)" round>DEL</base-button>
        </template>
      </el-table-column>

    </el-table>

      <div class="block" style="margin-top:15px;">
        <el-pagination align='center'
                       @size-change="handleSizeChange"
                       @current-change="handleCurrentChange"
                       :current-page="currentPage"
                       :page-sizes="[3,5,10]"
                       :page-size="pageSize"
                       layout="total, sizes, prev, pager, next, jumper"
                       :total="tableData.length">
        </el-pagination>
      </div>

    <dialog-component
        v-if="showDialog"
        ref="dialogComponent"
        :dialog-title="dialogTitle"
        :item-info="tableItem"
        :sendData="ifE"
        @closeDialog ="closeDialog"
        @pushDialogData="updateTable"
        @editDialog="updateEdit"
    ></dialog-component>

  </div>
</template>


<script>
import DialogComponent from "@/adminform/components/dialogComponent.vue";
import axios from "axios";
import event from "@/pages/Event/Event.vue";
import CardImg from "@/adminform/components/cardImg.vue";

export default {
  name: "DialogDemo",
  components: {CardImg, DialogComponent},
  data() {
    return {
      tableLoading: false,
      showDialog: false,
      dialogTitle: "PUBLISH EVENT",
      pageSize:5,
      currentPage: 1,
      ifE: false,
      editrowNum:'0',
      tableData: [
        // {activityName:'ddd', kind:'Free', venue_id:'1', description:'4'},
        // {activityName:'ddd', kind:'Free', venue_id:'3', description:'4'},
      ],
      tableItem: {
        activityName:'',
        description:'',
        kind:'',
        cover:null,
        venue_id:0,
        location:[0.0,0.0],
        startTime:'2023-03-02 08:08',
        endTime:'',
        deadline: '',
        id: 0,
      },
    };
  },

  mounted() {
    this.fetchData();
    this.$nextTick(() => { //挂载组件的时候
      const role = localStorage.getItem("role");
      if (role !== 'admin') {
        this.showFailMessage("you have no access to this page");
        window.location.href = "#/dashboard/dashboard";
      }
    });
  },

  computed: {
    filteredTableData() {
      const startIndex = (this.currentPage - 1) * this.pageSize;
      const endIndex = startIndex + this.pageSize;
      return this.tableData.slice(startIndex, endIndex);
    }
  },

  methods: {
    fetchData() {
      const that = this;
      that.tableLoading = true;
      setTimeout(() => {
        that.tableLoading = false;
      }, 1000);

      const apiUrl = "https://backend.sustech.me/api/event"
      axios.get(apiUrl, {timeout: 3000})
          .then(response => {
            const events = response.data.events;
            this.tableData = [];
            for (const event of events) {
              let get_event = {};
              get_event.id = event.id;
              get_event.activityName = event.name;
              get_event.kind = event.kind;
              get_event.description = event.description;
              get_event.cover=event.cover;
              get_event.startTime = event.start_at;
              get_event.endTime = event.end_at; //string
              get_event.venue_id = event.venue.id; //string
              get_event.location = event.location;
              // get_event.latitude = event.location[0];
              // get_event.longtitude = event.location[1];
              get_event.tickets = event.tickets;
              get_event.deadline = event.registeration_deadline;
              // console.log("event: ",event);
              this.tableData.push(get_event);
            }
          })
          .catch(error => {
            console.error('failed to init activities：', error);
            this.showFailMessage(`活动初始化失败`);
          })
    },
    handleSizeChange(val) {
      console.log(`${val} records per page`);
      this.currentPage = 1;
      this.pageSize = val;
    },
    //当前页改变时触发 跳转其他页
    handleCurrentChange(val) {
      console.log(`current page: ${val}`);
      this.currentPage = val;
    },

    addItem() {
      this.tableItem={
        activityName:'',
        description:'',
        cover: null,
        kind:'',
        venue_id:'',
        location: [],
        // latitude:0.0,
        // longtitude: 0.0,
        tickets:'',
        startTime:'',
        endTime:'',
        deadline:'',
        id:'',
      };
      this.dialogTitle = "publish";
      this.showDialog = true;
      this.$nextTick(() => {
        this.$refs["dialogComponent"].showDialog = true;
      });
    },

    editRow(ind,row) {
      this.tableItem = row;
      this.dialogTitle = "edit";
      this.ifE=true;
      this.editrowNum=ind;
      this.showDialog = true;
      this.$nextTick(() => {
        this.$refs["dialogComponent"].showDialog = true;
        //this.$refs["dialogComponent"].father_ifE=true;
      });
    },

    updateTable(){
      // this.tableData.push(data);
      this.fetchData();
    },
    updateEdit(data){
      //this.tableData[this.editrowNum]=data;
      console.log("edit")
      console.log(this.editrowNum)
      this.fetchData();
      this.tableData.splice(this.editrowNum,1,data)
      this.$nextTick(() => {
      });
      // console.log(this.tableData)
    },

    closeDialog(flag) {
      if (flag) {
        this.fetchData();
      }
      this.showDialog = false;
      // this.fetchData();
    },

    deleteRow(index,ifMsg){
      // this.tableData.splice(index, 1);
      const event_id = this.tableData.at(index).id;
      const deleteUrl = `https://backend.sustech.me/api/event/${event_id}`
      axios.delete(deleteUrl, {timeout: 3000,headers: {
              Authorization: `Bearer ${localStorage.getItem('token')}`
            }
          })
          .then(response => {
            if (ifMsg) {
              this.$message({
                message: `删除成功`,
                type: "success",
              });
              // this.fetchData();
            }
          })
          .catch(error => {
            console.error('failed to delete event：', error);
            this.showFailMessage(`删除活动失败`);
            return false;
          })
      this.fetchData();
      return true;
    },

    showFailMessage(message) {
      this.Message = message;
      this.$message({
        message: this.Message,
        type: 'error'
      });
    },

    handleSelectionChange(selection) {
      this.selectedRows = selection;
    },
    deleteSelectedRows() {
      const indices = this.selectedRows.map(row => this.tableData.indexOf(row));
      // if (indices.length > 0) {
      //   this.tableData = this.tableData.filter((_, index) => !indices.includes(index));
      // }
      let check = true;
      for (const idx of indices){
        check = check && this.deleteRow(idx,false);
      }
      if (check) {
        this.showSuccessMessage(`删除选择部分活动成功`);
        this.fetchData();
      }else{
        this.showFailMessage(`不能删除非您发布的活动`);
      }
    },
    showSuccessMessage(message) {
      this.successMessage = message;
      this.$message({
        message: this.successMessage,
        type: 'success'
      });
    },
    deleteAllRows() {
      // this.tableData = [];
      let check = true;
      for (let i = 0; i < this.tableData.length; i++) {
        check= check && this.deleteRow(i, false);
      }
      if (!check) {
        this.showFailMessage(`不能删除非您发布的活动`);
      }
      this.fetchData();
    },

  },
};
</script>


<style scoped lang="scss">
@import url('https://fonts.googleapis.com/css2?family=Lobster&display=swap');

.dialog-demo{
  position: absolute;
  padding: 50px 115px;
  justify-content: left; // center
  .instructions {
    font-size: 20px;
    padding: 10px 0;
    color: #304455;
  }
  .desc-list {
    padding: 10px 0 30px 40px;
    line-height: 30px;
    font-size: 15px;
    color: #606266;
    list-style-type: disc;
  }
}

.storeTable{
  width: 210px;
  background-color: rgba(255, 255, 255,0.6);
  opacity: 0.96;
  font-size: 15px;
  font-family: 'EB Garamond', sans-serif; /* 使用艺术字体 */
}

  .title{
    animation-name: colorChange;
    animation-duration: 6s;
    animation-iteration-count: infinite;
    font-size: 35px;
    font-weight: bolder;
    //text-shadow: -3px -3px 0px rgba(0, 0, 255, 0.2),
  }

  @keyframes colorChange {
    0% { color: #b895d5; }
    50% { color: #a16cc5; }
    100% { color: #805bb6; }
  }

</style>