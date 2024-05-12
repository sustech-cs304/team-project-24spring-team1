<template>
  <div class="row">
    <div class="col-md-8">
      <publish-form :activity="activity"> </publish-form>
    </div>
<!--    <img class="background" :src="imgUrl" width="100%" height="100%" alt=""/>-->
  </div>
</template>

<script>
import PublishForm from "@/pages/Admin/PublishForm.vue";

export default {
  components: {
    PublishForm,
  },
  // 在Vue中，组件名是大小写敏感的，但是在模板中使用组件时，Vue会自动将组件名转换为小写形式。这是因为在HTML中，标签名是不区分大小写的，所以Vue在模板编译过程中会将组件名统一转换为小写形式，以确保能够正确识别和匹配组件。
  data() {
    return { // volunteer
      activity: {
        datetime: "mike@email.com",
        name: "michael23",
        place: "Bld Mihail Kogalniceanu, nr. 8 Bl 1, Sc 1, Ap 09",
        capacity: "Melbourne",
        holder: "Australia",
        details:
            "Lamborghini Mercy, Your chick she so thirsty, I'm in that two seat Lambo.",
        IFfree: false,
        contacts: "15630230844",
      },
    };
  },
};
</script>
<style></style>
