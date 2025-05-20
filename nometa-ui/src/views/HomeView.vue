<script setup lang="ts">
import { ref, onMounted } from 'vue';
import TerminalText from '@/components/TerminalText.vue'

const isLoaded = ref(false);

onMounted(() => {
  window.addEventListener('load', () => {
      isLoaded.value = true;
  });
});
</script>

<template>
<body>
    <!-- <Transition name="fade">
        <div v-if="isLoaded" class="landing">
            <TerminalText text="Welcome to nometa.id" :typing-speed="100" />
        </div>
    </Transition> -->
</body>
    
</template>

<style scoped>
body {
   margin: 2rem;
}

.landing {
    display: flex;
    justify-content: center;
    font-size: 32px;
}

</style>
