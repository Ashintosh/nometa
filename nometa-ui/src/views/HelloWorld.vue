<script setup lang="ts">
import { ref, onMounted } from 'vue';

const message = ref('Loading...');

onMounted(async () => {
    try {
        const response = await fetch('http://localhost:8080/api/hello');
        const data = await response.json();
        message.value = data.message;
    } catch (error) {
        message.value = 'Error connecting to the backend';
        console.error(error);
    }
});
</script>

<template>
    <div>
        <h1>{{ message }}</h1>
        <p>This message comes from the Rust backend.</p>
    </div>
</template>