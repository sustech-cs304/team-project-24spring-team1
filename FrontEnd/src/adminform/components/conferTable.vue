<script>
//import {ref} from 'vue'

export default {
  el: '#location-vue',
  data() {
    return {
      activityName: '',
      kind: '',
      description: '',
      startTime: '',
      endTime: '',
      venue_id: '',
      deadline: '',
      tickets: '',
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
  // let Regex = new RegExp(/^[A-Za-z]+$/);
  let nameRegex =new RegExp(/^[A-Za-z]+\d+$/);
  let locationRegex =new RegExp(/^\d+[A-E]+$/);
  let ticketRegex=new RegExp(/^\d+$/);
  if (!nameRegex.test(this.activityName)) {
    alert("Invalid activity name.");
    return false;
  }
  if (!locationRegex.test(this.location)) {
    alert("Invalid location.");
    return false;
  }
  if(!ticketRegex.test(this.tickets)){
    alert("invalid number of tickets");
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
          <th>Activity Name</th>
          <th>Description</th>
          <th>kind</th>
          <th>Start Time</th>
          <th>End Time</th>
          <th>Tickets</th>
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
          <td>
            <input type="button" value="Delete" onclick="removeRow(this)">
            <input type="button" value="Edit" onclick="editRow(this)"/>
          </td>
        </tr>
        </tbody>
      </table>
      </div>


      <div class="input-wrapper">
        <form ref="fill-in-form">
          <h3>Add Activity</h3>
          <div class="border-wrapper">
            <label class="my-label">Activity Name</label>
            <input type="text" name="activity-name" v-model="activityName" class="border-item" placeholder="activityName"/>
          </div>

          <div class="selector">
            <label class="my-label">Type</label>
            <input type="radio" class="selector" id="small" name="room-type" value="small" v-model="kind"/>Free
            <input type="radio" class="selector" id="medium" name="room-type" v-model="kind"/>fullyPaid
            <input type="radio" class="selector" id="big" name="room-type" v-model="kind"/>SchoolDiscount
            <!-- :=是什么  -->
          </div>

          <label class="border-wrapper">
            <label class="my-label">Department</label>
            <input type="text" v-model="description" name="description" class="border-item" placeholder="description"/>
          </label>

          <label class="border-wrapper">Start Time
            <input type="datetime-local" v-model="startTime" name="start-time" class="selector"/>
          </label>
          <label class="border-wrapper">End Time
            <input type="datetime-local" v-model="endTime" name="end-time" class="selector"/>
          </label>

          <label class="border-wrapper" >
            Venue
            <select name="fruit" class="selector" v-model="venue_id"> <!--v-for="(key,value) in loclist" -->
              <option>"--"</option>
              <option>"Teaching Building No.1"</option>
              <option>"Lecture Hall"</option>
              <option>"Research Building Lecture Hall"</option>
              <option>"Library Conference Hall"</option>
              <option>"South Building</option>
            </select>
            <input type="text" v-model="venue_id" id="location-layer" placeholder="venue of the activity" class="border-item"/>
          </label>

          <label class="border-wrapper">Deadline
            <input type="datetime-local" v-model="deadline" name="deadline" class="border-item"/>
          </label>

          <label class="border-wrapper">Deadline
            <input type="datetime-local" v-model="tickets" name="tickets" class="border-item"/>
          </label>

          <div class="action">
            <input type="button" id="leftButton" @click="onClickAddConfer" class="btn" align="left" v-model="addbtn"/>
            <div></div>
            <input type="button" id="rightButton" @click="clearEntry" class="btn" align="right" v-model="clearbtn"/>
          </div>

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