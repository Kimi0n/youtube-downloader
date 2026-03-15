<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import './assets/style.css'

const youtubeLink = ref<string>('');
const youtubeID = ref<string>('');
const statusMessage = ref<string>('');
const isReady = ref<boolean>(false);

async function prepareForDownloading() {
  try {
    statusMessage.value = `Downloading...`;
    isReady.value = false;

    await invoke('download_from_youtube', { 
      youtubeId: youtubeID.value
    });

  } catch (error) {
    statusMessage.value = `Error: ${error}`;
    if( youtubeID.value != `` ) { isReady.value = true; }
  }
}

async function checkLinkValidity() {
  const regex = /^(?:https?:\/\/)?(?:www\.)?(?:youtu\.be\/|youtube\.com\/(?:embed\/|v\/|watch\?v=|watch\?.+&v=|shorts\/))((\w|-){11})(?:\S+)?$/;

  if(youtubeLink.value == ``) {
    youtubeID.value = ``;
    statusMessage.value = ``;
    isReady.value = false;
    return;
  }

  const matches = youtubeLink.value.match(regex);

  if(matches) {
    youtubeID.value = matches[1];
    statusMessage.value = ``;
    isReady.value = true;
  } else {
    youtubeID.value = ``;
    statusMessage.value = `Invalid link!`;
    isReady.value = false;
  }
}

listen('yt-dlp-progress', (event) => {
  statusMessage.value = `${event.payload}`;
});

listen('yt-dlp-finished', (event) => {
  const conversionEndCode = event.payload; 

  if(conversionEndCode == 0) {
    statusMessage.value = `Video downloaded!`;
  }

  if( youtubeID.value != `` ) { isReady.value = true; }
});
</script>

<template>
  <main class="container">
    <p>{{ statusMessage || '&nbsp;' }}</p>
    <div>
      <input v-model="youtubeLink" @input="checkLinkValidity" placeholder="Enter a Youtube link..." />
      <button :disabled="!isReady" type="submit" @click="prepareForDownloading">Download</button>
    </div>
  </main>
</template>