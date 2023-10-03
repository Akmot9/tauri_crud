<template>
    <div>
      <h1>Packet Information</h1>
      <button @click="createTable">Get</button>
      <ul>
        <li v-for="packetInfo in packetInfoList" :key="packetInfo.id">
          <span><strong>MAC Source:</strong> {{ packetInfo.mac_source }}</span><br>
          <span><strong>MAC Destination:</strong> {{ packetInfo.mac_destination }}</span><br>
          <span><strong>IPv4:</strong> {{ packetInfo.ethertype }}</span><br>
          <span><strong>IP Source:</strong> {{ packetInfo.ip_source }}</span><br>
          <span><strong>IP Destination:</strong> {{ packetInfo.ip_destination }}</span><br>
          <span><strong>Protocol:</strong> {{ packetInfo.protocol }}</span><br>
          <span><strong>Port Source:</strong> {{ packetInfo.port_source }}</span><br>
          <span><strong>Port Destination:</strong> {{ packetInfo.port_destination }}</span><br>
          <span><strong>Count:</strong> {{ packetInfo.count }}</span><br>
        </li>
      </ul>
    </div>
  </template>
  
  
<script>
  import { ref } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';
  
  export default {
    data() {
      return {
        packetInfoList: [], // Store retrieved packet information here
      };
    },
    methods: {
      async createTable() {
        try {
          // Now fetch the data from the database using another command
          this.packetInfoList = await invoke('get_packet_infos');
          
        } catch (error) {
          console.error(error);
        }
      },
    },
  };
</script>
  