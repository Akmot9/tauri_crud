<template>
    <div>
      <h1>Create Table</h1>
      <button @click="createTable">Create Table</button>
    </div>
  </template>
  
  <script>
  import { ref } from 'vue';
  import { invoke } from '@tauri-apps/api/tauri';
  
  export default {
    methods: {
      async createTable() {
        try {
          const response = await invoke('create_table');
          console.log('Table created:', response);
        } catch (error) {
          console.error('Error creating table:', error);
        }
      },
    },
  };
  </script>
  