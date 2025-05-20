<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { RouterLink, RouterView } from 'vue-router'
import TerminalText from '@/components/TerminalText.vue'

const isLoaded = ref(false);
const isMenuOpen = ref(false);

onMounted(() => {
  window.addEventListener('load', () => {
      isLoaded.value = true;
  });
});
</script>

<template>
  <Transition name="fade">
    <header v-if="isLoaded">
      <div class="title-text">
        <TerminalText text="nometa.id" :typing-speed="100" />
      </div>
      <!-- Regular nav menu -->
      <nav class="nav-bar">
        <RouterLink to="/login">Login</RouterLink>
        <RouterLink to="/signup">Sign Up</RouterLink>
      </nav>
      <!-- Hamburger nav menu for smaller screens -->
      <div class="nav-menu">
        <button class="hamburger-btn" @click="isMenuOpen = !isMenuOpen"
                :class="{ 'is-open': isMenuOpen }">
          <span class="hamburger-icon"></span>
        </button>
        <div class="hamburger-menu" :class="{ 'is-open': isMenuOpen }">
          <RouterLink to="/login" @click="isMenuOpen = false">Login</RouterLink>
          <RouterLink to="/signup" @click="isMenuOpen = false">Sign Up</RouterLink>
        </div>
      </div>
    </header>
  </Transition>
  <RouterView />
</template>

<style scoped>
header {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 5rem;
  padding-top: 4rem;
  padding-bottom: 4rem;
  gap: 15rem;
  border-bottom: 1px solid var(--color-primary);
}

.title-text {
  font-size: 32px;
  width: 400px;
}

.title-text :deep(.terminal-text) {
  font-family: 'SixtyFour';
}

.nav-bar {
  display: flex;
  gap: 1rem;
  justify-content: right;
}

.nav-bar a,
.nav-menu a {
  color: var(--color-primary);
  text-decoration: none;
  padding: 0.75rem 1.5rem;
  border: 1px solid var(--color-primary);
  /* border-radius: 4px; */
  transition: all 0.3s ease;
  font-size: 1.1rem;
  background-color: transparent;
}

.nav-bar a:hover,
.nav-menu a:hover {
  background-color: var(--color-text);
  color: var(--color-background);
  transform: translateY(-2px);
}

.nav-bar a:active,
.nav-menu a:active {
  color: var(--color-primary);
  border: 1px solid var(--color-primary);
  transition: all 0.15s ease;
}

.nav-bar a.router-link-exact-active,
.nav-bar a.router-link-active,
.nav-menu a.router-link-exact-active,
.nav-menu  a.router-link-active {
  background-color: #0004ff;
  color: #ff0000;
  border-color: #00ff0d;
}

.nav-menu {
  display: none;
}

.hamburger-btn {
  background: none;
  border: none;
  cursor: pointer;
  padding: 1rem;
}

.hamburger-btn.is-open .hamburger-icon {
  background-color: transparent;
}

.hamburger-btn.is-open .hamburger-icon::before {
  transform: rotate(45deg) translateY(0);
}

.hamburger-btn.is-open .hamburger-icon::after {
  transform: rotate(-45deg) translateY(0);
}

.hamburger-icon {
  display: block;
  width: 25px;
  height: 2px;
  background-color: var(--color-text);
  position: relative;
  transition: all 0.3s;
}

.hamburger-icon::before,
.hamburger-icon::after {
  content: '';
  position: absolute;
  width: 25px;
  height: 2px;
  background-color: var(--color-text);
  transition: all 0.3s;
}

.hamburger-icon::before {
  transform: translateY(-8px);
}

.hamburger-icon::after {
  transform: translateY(8px);
}

/* Mobile menu overlay */
.hamburger-menu {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  position: fixed;
  top: 10rem;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: var(--color-background);
  transform: translateX(100%);
  transition: transform 0.3s ease-in-out;
  opacity: 0;
  pointer-events: none;
}

.hamburger-menu.is-open {
  opacity: 1;
  pointer-events: auto;
  transform: translateX(0);
}

.hamburger-menu:not(.is-open) {
  transition: transform 0.3s ease-in-out,
              opacity 0.2s ease-in-out 0.2s;
}

@media (max-width: 880px) {
  header {
    justify-content: space-between;
    gap: 0;
  }

  .title-text {
    padding-left: 4rem;
  }

  .nav-bar {
    padding-right: 4rem;
  }
}

@media (max-width: 690px) {
  .title-text {
    padding-left: 2rem;
  }

  .nav-bar {
    display: none;
  }
  
  .nav-menu {
    display: block;
    padding-right: 2rem;
  }
}

@media (max-width: 450px) {
  .title-text {
    font-size: 20px;
    padding-left: 1rem;
  }
  
  .nav-menu {
    padding-right: 1rem;
  }
}
</style>