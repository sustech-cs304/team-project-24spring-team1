<script>
//import {ref} from 'vue'

export default {
  el: '#location-vue',
  data() {
    return {
      roomName: '',
      roomType: '',
      department: '',
      date: '',
      startTime: '',
      endTime: '',
      loc1: '',
      loc2: '', //ref(''),
      location:'',
      locSelect:'',
      locEntry:'',
      maxDuration: '',
      selected:''
    };
  },

  methods: {
    initial: function(){
      let year = this.$refs.date3['year'];
      year.innerHTML = "";
      year.options.add(new Option("--", null));
      for (let i = 2000; i <= 2020; i++) {
        year.options.add(new Option(i, i));
      }
    },
    removeRow: function(){

    } ,
    editRow: function (){

    },
    onClickAddConfer: function (){
      if(this.validateInput) {
        alert("success");
        //addRow();
      }else{
        alert("input format error");
      }
    },
    clearEntry() {
      Object.keys(this.data).forEach(key => {
        this.data[key] = '';
      });
      /*this.$refs["fill-in-form"].reset();*/
    },
    validateInput : function() {
  let deparRegex = new RegExp(/^[A-Za-z]+$/);
  let roomnameRegex =new RegExp(/^[A-Za-z]+\d+$/);
  let locationRegex =new RegExp(/^\d+[A-E]+$/);
  let durationRegex=new RegExp(/^\d+$/);
  if (!deparRegex.test(this.department)) {
    alert("Invalid Department input.");
    return false;
  }
  if (!roomnameRegex.test(this.roomName)) {
    alert("Invalid room name.");
    return false;
  }
  if (!locationRegex.test(this.location)) {
    alert("Invalid location.");
    return false;
  }
  if(!durationRegex.test(this.duration)){
    alert("invalid duration");
    return false;
  }
  return true;
}

  }
};
</script>


<template>
  <div class="wrap">
    <div class="header">conference room</div>
    <div class="form-wrapper">
      <div class="input-wrapper">
      <table border="1" cellpadding="10" cellspacing="5" id="form">
        <tr>
          <th>Room Name</th>
          <th>Department</th>
          <th>Type</th>
          <th>Location</th>
          <th>Date</th>
          <th>Start Time</th>
          <th>End Time</th>
          <th>Max Duration</th>
          <th>Operation</th>
        </tr>

        <tbody id="tbody" style="display: none">
        <tr>
          <td></td>
          <td></td>
          <td></td>
          <td></td>
          <td></td>
          <td></td>
          <td></td>
          <td></td>
<!--          <td>{{this.roomName}}}</td>-->
<!--          <td>{{this.department}}</td>-->
<!--          <td>{{item.roomType}}</td>-->
<!--          <td>{{item.location-selected}}</td>-->
<!--          <td>{{item.date}}</td>-->
<!--          <td>{{item.startTime}}</td>-->
<!--          <td>{{item.endTime}}</td>-->
<!--          <td>{{item.maxDuration}}</td>-->
          <td>
            <input type="button" value="Delete" onclick="removeRow(this)">
            <input type="button" value="Edit" onclick="editRow(this)"/>
          </td>
          <!--click vs onclick-->
        </tr>
        </tbody>
      </table>
      </div>


      <div class="input-wrapper">
        <form ref="fill-in-form">
          <h3>Add Room</h3>
          <div class="border-wrapper">
            <label class="my-label">Room Name</label>
            <input type="text" name="room-name" v-model="roomName" class="border-item" placeholder="roomName"/>
          </div>

          <div class="selector">
            <label class="my-label">Type</label>
            <input type="radio" class="selector" id="small" name="room-type" value="small" v-model="roomType"/>Small
            <input type="radio" class="selector" id="medium" name="room-type" v-model="roomType"/>Medium
            <input type="radio" class="selector" id="big" name="room-type" v-model="roomType"/>Big
            <!-- :=是什么  -->
          </div>

          <label class="border-wrapper">
            <label class="my-label">Department</label>
            <input type="text" v-model="department" name="department" class="border-item" placeholder="department"/>
          </label>

          <label class="border-wrapper">date
            <input type="date" v-model="date" min="2023-09-13" max="2023-10-12" id="my-date" class="selector"/>
            <!--unecessary value building: value="2023-09-12"-->
          </label>

          <!--为什么名称中间加"-"-->
          <label class="border-wrapper"> Start Time
            <input type="time" v-model="startTime" name="start-time" min="12:00 AM" class="selector"/>
          </label>
          <label class="border-wrapper">End Time
            <input type="time" v-model="endTime" name="end-time" class="selector"/>
          </label>


          <label class="border-wrapper" >
            Location
            <select name="fruit" class="selector" v-model="locSelect"> <!--v-for="(key,value) in loclist" -->
              <option>"--"</option>
              <option>"Teaching Building No.1"</option>
              <option>"Lecture Hall"</option>
              <option>"Research Building Lecture Hall"</option>
              <option>"Library Conference Hall"</option>
              <option>"South Building</option>
            </select>
            <input type="text" v-model="locEntry" id="location-layer" placeholder="detailed room name" class="border-item"/>
          </label>

          <label class="border-wrapper">Max Duration
            <input type="text" name="duration" v-model="maxDuration" class="border-item" placeholder="max duration"/>
          </label>


          <div class="action">
            <input type="button" id="leftButton" @click="onClickAddConfer" class="btn" align="left" v-model="addbtn"/>
            <div></div>
            <input type="button" id="rightButton" @click="clearEntry" class="btn" align="right" v-model="clearbtn"/>
          </div>

          <!--value干什么的-->
        </form>
      </div>
    </div>

  </div>
</template>


<style>
.wrap {
  height: 90%;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #5e93a1;
  background-size: 100% 100%;
}


.form-wrapper {
  width: 50%;
  background-color: rgba(83, 95, 145, 0.8);
  color: #fff;
  border-radius: 2px;
  padding: 35px;
}
.wrap .header {
  text-align: center;
  font-size: 25px;
  text-transform: uppercase;
  line-height: 70px;

}


.form-wrapper .input-wrapper input {
  background-color: rgb(78, 88, 129);
  border: 0;
  width: 60%;
  text-align: center;
  font-size: 10px;
  color: #fff;
  outline: none;
  border-radius: 30px;
}
.form-wrapper .input-wrapper input::placeholder {
  text-transform: uppercase;
}


.form-wrapper .input-wrapper .border-wrapper {
  background-image: linear-gradient(to right, #e8198b, #0eb4dd);
  width: 85%;
  height: 30px;
  margin-bottom: 30px;
  border-radius: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
}


.form-wrapper .input-wrapper .border-wrapper .border-item {
  height: calc(100% - 4px);
  width: calc(100% - 5px);
  border-radius: 30px;
}
.form-wrapper .input-wrapper .border-wrapper .selector{
  background-image: linear-gradient(to right, #e8198b, #0eb4dd);
  width: calc(70% - 4px);
  height:calc(90%- 3px);
  font-size: 15px;
  border-radius: 30px;
  display: inline-block;
}



.form-wrapper .action {
  display: flex;
  justify-content: center;
}
.form-wrapper .action .btn {
  width: 16%;
  text-transform: uppercase;
  border: 2px solid #0e92b3;
  text-align: center;
  line-height: 25px;
  border-radius: 20px;
  cursor: pointer;
  display: inline-block;
  margin: 0 30px;
  /*margin-top: 18px; /* 设置合适的上部外框的宽度，增加与上面的password框的距离
  transition:0.2s;  圆角弧度24px */
}
.form-wrapper .action .btn:hover {
  background-image: linear-gradient(120deg, #84fab0 0%, #8fd3f4 100%);
}
/*加border-wrapper没报错*/

.form-wrapper .icon-wrapper {
  text-align: center;
  width: 40%;
  margin: 0 auto;
  /*margin-top: 20px;*/
  border-top: 1px dashed rgb(146, 146, 146);
  padding: 20px;
}
.form-wrapper .icon-wrapper i {
  font-size: 20px;
  color: rgb(187, 187, 187);
  cursor: pointer;
  border: 1px solid #fff;
  padding: 5px;
  border-radius: 20px;
}
.form-wrapper .icon-wrapper i:hover {
  background-color: #0e92b3;
}

</style>