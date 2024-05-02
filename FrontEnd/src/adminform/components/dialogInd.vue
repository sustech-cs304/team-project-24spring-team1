<template>
  <div class="dialog-demo" :style="{ backgroundImage:  'url(' + imgUrl + ')' }">
    <!--el-divider content-position="left">conference table</el-divider-->
      <p class="title"> conference room </p>
    <el-row>
      <el-button type="primary" @click="addItem" size="medium">NEW</el-button>
      <el-button type="primary" @click="deleteSelectedRows" size="medium" style="margin-left:40px;">
        delete selection</el-button>
      <!--el-button type="primary" @click="selectAll" size="medium" style="margin-left:40px;">select all</el-button-->
      <el-button type="primary" @click="deleteAllRows" size="medium" class="primary"
                 style="margin-left:40px; background-color: #fa3434;">delete all</el-button>
    </el-row>

    <el-table
        v-loading="tableLoading"
        :data="tableData.slice((currentPage-1)*pageSize,currentPage*pageSize)"
        :page-size="5" stripe pagination
        class="storeTable"
        style="width: 98%; margin-top: 6px;"
        @selection-change="handleSelectionChange"
    >
      <!--"v-for="item in tableItem" :key="checked"-->
      <el-table-column label="select" width="80" type="selection"></el-table-column>

      <el-table-column label="roomName" prop="roomName"></el-table-column>
      <el-table-column label="department" prop="department"></el-table-column>
      <el-table-column label="roomType" prop="roomType"></el-table-column>
      <el-table-column label="Date" prop="date"></el-table-column>
      <el-table-column label="startTime" prop="startTime"></el-table-column>
      <el-table-column label="endTime" prop="endTime"></el-table-column>
      <el-table-column label="location" prop="loc1" width="160"></el-table-column>
      <el-table-column prop="loc2"></el-table-column>
      <el-table-column label="duration" prop="duration" width="100"></el-table-column>

      <el-table-column label="operation" width="120">
        <template slot-scope="scope">
          <el-button type="primary" size="small" id="editBtn"
                     @click.native.prevent="editRow(scope.$index,scope.row)" plain>EDIT</el-button>
          <el-button type="primary" size="small" id="deleteBtn" style="background-color: red;text-decoration-color: #e8198b"
                     @click.native.prevent="deleteRow(scope.$index)" round>DELETE</el-button>
        </template>
      </el-table-column>
      <!--index需要加$-->

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
import DialogComponent from "@/components/dialogComponent.vue";

export default {
  name: "DialogDemo",
  components: { DialogComponent},
  data() {
    return {
      tableLoading: false,
      showDialog: false,
      dialogTitle: "添加会议",
      pageSize:5,
      currentPage: 1,
      ifE: false,
      editrowNum:'0',
      tableData: [
        {department:'ddd', roomType:'small', loc1:'', loc2:'4A',duration:'4'},
        {department:'ced', roomType:'big', loc1:'', loc2:'',duration:'4'},
        {department:'ddd', roomType:'small', loc1:'', loc2:'',duration:'6'},
        {department:'dv', roomType:'medium', loc1:'', loc2:'',duration:'7'},
        {department:'dvfd', roomType:'small', loc1:'', loc2:'',duration:''},
        {department:'dd', roomType:'big', loc1:'', loc2:'',duration:''},
        {department:'defv', roomType:'small', loc1:'', loc2:'',duration:'8'},
      ],
      tableItem: {
        roomName:'',
        department:'',
        roomType:'',
        loc1:'',
        loc2:'',
        date:'',
        startTime:'08:08',
        endTime:'',
        duration:'',
      },
    };
  },
  mounted() {
    this.fetchData();
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
      console.log(this.tableData)
      setTimeout(() => {
        that.tableLoading = false;
      }, 1000);
    },
    handleSizeChange(val) {
      console.log(`每页 ${val} 条`);
      this.currentPage = 1;
      this.pageSize = val;
    },
    //当前页改变时触发 跳转其他页
    handleCurrentChange(val) {
      console.log(`当前页: ${val}`);
      this.currentPage = val;
    },

    addItem() {
      this.tableItem={
        roomName:'',
        department:'',
          roomType:'',
          loc1:'',
          loc2:'',
          date:'',
          startTime:'',
          endTime:'',
          duration:'',
      };
      this.dialogTitle = "添加";
      this.showDialog = true;
      this.$nextTick(() => {
        this.$refs["dialogComponent"].showDialog = true;
      });
    },

    editRow(ind,row) {
      this.tableItem = row;
      this.dialogTitle = "编辑";
      this.ifE=true;
      this.editrowNum=ind;
      this.showDialog = true;
      this.$nextTick(() => {
        this.$refs["dialogComponent"].showDialog = true;
        //this.$refs["dialogComponent"].father_ifE=true;
      });
    },

    updateTable(data){
      this.tableData.push(data);
    },

    updateEdit(data){
      //this.tableData[this.editrowNum]=data;
      console.log("edit")
      console.log(this.editrowNum)
      this.tableData.splice(this.editrowNum,1,data)
      this.$nextTick(() => {

      });
      console.log(this.tableData)
    },

    closeDialog(flag) {
      if (flag) {
        this.fetchData();
      }
      this.showDialog = false;
    },
    deleteRow(index){
      this.tableData.splice(index, 1);
    },
    handleSelectionChange(selection) {
      this.selectedRows = selection;
    },
    deleteSelectedRows() {
      const indices = this.selectedRows.map(row => this.tableData.indexOf(row));
      if (indices.length > 0) {
        this.tableData = this.tableData.filter((_, index) => !indices.includes(index));
      }
    },
    deleteAllRows() {
      this.tableData = [];
    },

  },
};
</script>


<style scoped lang="scss">
.dialog-demo{
  position: absolute;
  padding: 20px;
  .instructions {
    font-size: 14px;
    padding: 10px 0;
    color: #304455;
  }
  .desc-list {
    padding: 10px 0 30px 40px;
    line-height: 30px;
    font-size: 14px;
    color: #606266;
    list-style-type: disc;
  }

}

.storeTable{
  width: 150px;
  background-color: rgba(255, 255, 255,0.6);
  opacity: 0.95;
}

  .title{
    animation-name: colorChange;
    animation-duration: 3s;
    animation-iteration-count: infinite;
    font-size: 40px;
    font-weight: bolder;
    text-shadow:
        3px 3px 1px rgba(255, 0, 0, 0.2),
        -3px -3px 0px rgba(0, 0, 255, 0.2),
        3px -3px 0px rgba(0, 255, 0, 0.2),
        -3px 3px 0px rgba(255, 255, 0, 0.2);
  }

  @keyframes colorChange {
    0% { color: red; }
    50% { color: blue; }
    100% { color: green; }
  }

</style>