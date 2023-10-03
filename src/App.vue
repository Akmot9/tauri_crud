<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import Get_table from "./components/Get_table.vue";
import AddPacketInfo from "./components/AddPacketInfo.vue";
import creat_table from "./components/creat_table.vue"
</script>

<template>
  <div>

    <Greet />
    <Get_table />
    <AddPacketInfo />
    <creat_table />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
