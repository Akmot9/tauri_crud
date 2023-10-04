<template>
    <div>
      <h2>Update Packet Information</h2>
      <form @submit.prevent="submitForm">
        <div class="form-group">
          <label for="mac_source">MAC Source:</label>
          <input type="text" id="mac_source" v-model="packetInfo.mac_source" required />
        </div>
        <div class="form-group">
          <label for="count">New count:</label>
          <input type="number" id="count" v-model="packetInfo.count" required />
        </div>
        <!-- Add other form fields for PacketInfo properties -->
        <button type="submit">Update Packet Info</button>
      </form>
    </div>
  </template>
  
  <script>
  import { invoke } from '@tauri-apps/api'

  export default {
    data() {
      return {
        packetInfo: {
          mac_source: "",
          mac_destination: "",
          ethertype: "",
          ip_source: "",
          ip_destination: "",
          protocol: "",
          port_source: "",
          port_destination: "",
          count: 0,
        },
      };
    },
    methods: {
      async submitForm() {
        try {
          // Send the packetInfo object to Rust for insertion
          invoke('update_packet_info', {new_packet_info: this.packetInfo})
          .then((response) => console.log(response))
        } catch (error) {
          console.error(error);
          alert("An error occurred while adding packet info.");
        }
      },
    },
  };
  </script>
  
  <style scoped>
  /* Add your component-specific styles here */
  </style>
  