<template>
  <div class="container">
    <header>
      <h1>Tauri CRUD Example</h1>
      <p>A simple CRUD application using Tauri and Rust's Rusqlite</p>
    </header>
    <main>
      <section>
        <h2>Get Table Component</h2>
        <GetTable />
      </section>

      <section>
        <h2>Add Packet Info Component</h2>
        <AddPacketInfo />
      </section>

      <section>
        <h2>Create Table Component</h2>
        <CreatTable />
      </section>
    </main>
  </div>
</template>

<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import GetTable from "./components/GetTable.vue";
import AddPacketInfo from "./components/AddPacketInfo.vue";
import CreatTable from "./components/CreatTable.vue";
</script>

<style scoped>
.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

header {
  text-align: center;
  margin-bottom: 20px;
}

h1 {
  font-size: 28px;
}

p {
  font-size: 16px;
  color: #666;
}

section {
  border: 1px solid #ccc;
  padding: 20px;
  margin-bottom: 20px;
}

h2 {
  font-size: 20px;
  margin-bottom: 10px;
}
</style>
