<template>
  <div class="dialog-demo">
<!--       :style="{ backgroundImage:  'url(' + imgUrl + ')' }">-->
    <p class="title"> Activity Management </p>
    <el-row style="margin-top: 25px;margin-bottom: 10px">
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
      <el-table-column label="activityName" prop="activityName" width="110"></el-table-column>
      <el-table-column label="kind" prop="kind" width="105"></el-table-column>
      <el-table-column label="venue_id" prop="venue_id" width="110"></el-table-column>
      <el-table-column label="startTime" prop="startTime" width="110"></el-table-column>
      <el-table-column label="endTime" prop="endTime" width="110"></el-table-column>
      <el-table-column label="deadline" prop="duration" width="100"></el-table-column>
      <el-table-column label="description" prop="description" width="150"></el-table-column>

      <el-table-column label="operation" width="100">
        <template slot-scope="scope">
          <base-button type="primary" size=sm id="editBtn"
                     @click.native.prevent="editRow(scope.$index,scope.row)" round>EDIT</base-button>
          <base-button type="primary" size=sm id="deleteBtn" style="background-color: red"
                     @click.native.prevent="deleteRow(scope.$index)" round>DEL</base-button>
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

export default {
  name: "DialogDemo",
  components: { DialogComponent},
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
        venue_id:'',
        startTime:'2023-03-02 08:08',
        endTime:'',
        deadline: '',
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
        kind:'',
        venue_id:'',
        tickets:'',
        startTime:'',
        endTime:'',
        deadline:'',
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
  opacity: 0.95;
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