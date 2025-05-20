<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import TerminalText from '@/components/TerminalText.vue'
</script>

<template>
    
    <RouterView />
</template>

<style>
/* Remove default margins and padding */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  margin: 0;
  padding: 0;
  background-color: #1a1a1a;
}
</style>
