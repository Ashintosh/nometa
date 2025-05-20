<script setup lang="ts">
import { ref, onMounted, defineComponent } from 'vue';

defineComponent({
  name: 'TerminalText'
});

const props = defineProps<{
  text: string;
  typingSpeed?: number;
}>();

const emit = defineEmits<{
  (e: 'typing-complete'): void
}>();

const displayedText = ref('');
const showCursor = ref(true);

onMounted(() => {
  let currentIndex = 0;
  const interval = setInterval(() => {
    if (currentIndex < props.text.length) {
      displayedText.value += props.text[currentIndex];
      currentIndex++;
    } else {
      clearInterval(interval);
      emit('typing-complete');
    }
  }, props.typingSpeed || 100);
});
</script>

<template>
  <span class="terminal-text">{{ displayedText }}<span class="cursor" :class="{ blink: showCursor }">_</span></span>
</template>

<style scoped>
.terminal-text {
  display: inline-block;
  font-family: 'KodeMono';
}

.cursor {
  display: inline-block;
  margin-left: 2px;
}

.cursor.blink {
  animation: blink 1s step-end infinite;
}

@keyframes blink {
  50% {
    opacity: 0;
  }
}
</style>