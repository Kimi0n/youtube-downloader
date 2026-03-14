<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import './assets/style.css'

const youtubeLink = ref<string>('');
const statusMessage = ref<string>('');

async function prepareForDownloading() {
  try {
    await invoke('download_from_youtube', { 
      youtubeLink: youtubeLink.value
    });

  } catch (error) {
    statusMessage.value = "Error: " + error;
  }
}
</script>

<template>
  <main class="container">
    <p>{{ statusMessage }}</p>
    <div>
      <input v-model="youtubeLink" placeholder="Enter a Youtube link..." />
      <button type="submit" @click="prepareForDownloading">Download</button>
    </div>
  </main>
</template>