<script setup lang="ts">
import { ref } from 'vue';
import viteUrl from '../assets/vite.svg'
import vueUrl from '../assets/vue.svg'
const message = ref(null);

async function getMessage() {
    const response = await fetch('http://localhost:3030');
    const json = await response.json();
    message.value = json.message;
}
</script>

<template>
  <div style="container">
    <h1 style="font-size:48px">
      Rust &#129408;
      + Vue <img :src="vueUrl" alt="vueLogo"/>
      + Vite <img :src="viteUrl" alt="viteLogo"/>
    </h1>
  </div>
  <p>Hello World!</p>
  <div v-if="message">
    Received: {{ message }}
  </div>
  <div v-else>
    No message
  </div>
  <button @click="getMessage()">Get message</button>
</template>

<style scoped>
.container {
  display: inline-block;
  align-items: center;
  justify-content: center
}

img {
  width: 48px;
}
</style>
