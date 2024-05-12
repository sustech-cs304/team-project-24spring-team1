<!-- eslint-disable vue/no-mutating-props -->
<template>
  <card>
    <h5 slot="header" class="title">Activity Information</h5>
    <div class="row">

      <div class="col-md-5 pr-md-1">
        <base-input
          label="Company (disabled)"
          placeholder="Company"
          v-model="activity.name"
          disabled
        >
        </base-input>
      </div>

      <div class="col-md-3 px-md-1">
        <base-input
          label="Holder"
          placeholder="Holder"
          v-model="activity.holder"
        >
        </base-input>
      </div>

      <div class="col-md-4 pl-md-1">
        <base-input
          label="Email address"
          type="email"
          placeholder="mike@email.com"
        >
        </base-input>
      </div>
    </div>

    <div class="row">
      <div class="col-md-6 pl-md-1">
        <base-input
          label="contacts"
          v-model="activity.contacts"
          placeholder="Contacts"
        >
        </base-input>
      </div>
    </div>

    <div class="row">
      <div class="col-md-12">
        <base-input
          label="Address"
          v-model="activity.capacity"
          placeholder="Home Address"
        >
        </base-input>
      </div>
    </div>
    <div class="row">
      <div class="col-md-4 px-md-1">
        <base-input
          label="Country"
          v-model="activity.place"
          placeholder="Country"
        >
        </base-input>
      </div>
      <div class="col-md-4 pl-md-1">
        <base-input label="Postal Code" placeholder="ZIP Code"> </base-input>
      </div>
    </div>
    <div class="row">
      <div class="col-md-8">
        <base-input>
          <label>About Me</label>
          <textarea
            rows="4"
            cols="80"
            class="form-control"
            placeholder="Here can be your description"
            v-model="activity.details"
          >
          </textarea>
        </base-input>
      </div>
    </div>
    <base-button slot="footer" type="primary" fill> Publish </base-button>
  </card>
</template>

<script>
export default {
  props: {
    activity: {
      type: Object,
      default: () => {
        return {};
      },
    },
  },
};
</script>
<style></style>
