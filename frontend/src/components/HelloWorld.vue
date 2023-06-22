<script setup lang="ts">
import { ref } from 'vue';
import vueUrl from '../assets/vue.svg'
const message = ref("");
const input = ref("");

async function getMessage() {
    const response = await fetch('http://localhost:3030/hello');
    const json = await response.json();
    message.value = "Received: " + json.message;
}

async function getMessageWithName() {
  if (input.value.length > 0) {
    const response = await fetch('http://localhost:3030/hello/' + input.value);
    const json = await response.json();
    message.value = "Received: " + json.message;
  } else {
    message.value = "Please provide a name";
  }
}
</script>

<template>
  <div style="container">
    <h1 style="font-size:48px">
      Rust &#129408;
      + Vue <img :src="vueUrl" alt="vueLogo"/>
      + Vite <img src=vite.svg alt="viteLogo"/>
    </h1>
  </div>
  <p>Hello World!</p>
  <div style="color:greenyellow" v-if="message && message != 'Please provide a name'">
    {{ message }}
  </div>
  <div style="color:orange" v-else-if="message">
    {{ message }}
  </div>
  <div style="color:orange" v-else>
    No message
  </div>
  <button class="button" @click="getMessage()">GET MESSAGE</button>
  <button class="button" @click="getMessageWithName()">GET NAME</button>
  <div style="row">
    <input class="input" v-model="input"/>
  </div>
</template>

<style scoped>
.container {
  display: inline-block;
  align-items: center;
  justify-content: center
}

.button {
  padding: 0 10px 0 10px;
  height: 30px;
  margin: 20px 10px 0 10px;
  border-radius: 2px;
  font-weight: 700;
  width: 150px;
  color: #0f0e1f;
  background-color: white;
}

.input {
  padding: 0 10px 0 10px;
  height: 30px;
  margin-top: 20px;
  max-width: 125px;
  font-weight: bold;
}

img {
  width: 48px;
}
</style>
