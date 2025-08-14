<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'

const isOpen = ref(false)
const windowWidth = ref(window.innerWidth)
const MOBILE_BREAKPOINT = 800

const toggleMenu = () => {
  isOpen.value = !isOpen.value
}

const handleResize = () => {
  windowWidth.value = window.innerWidth
  if (windowWidth.value >= MOBILE_BREAKPOINT) {
    isOpen.value = false
  }
}

onMounted(() => {
  window.addEventListener('resize', handleResize)
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', handleResize)
})
</script>

<template>
  <header>
    <nav class="navbar py-3 shadow-sm">
      <div class="container d-flex align-items-center justify-content-between">
        <!-- Left nav (desktop only) -->
        <ul v-if="windowWidth >= MOBILE_BREAKPOINT" class="nav nav-left">
          <li class="nav-item">
            <RouterLink class="nav-link" to="/contact">Contact</RouterLink>
          </li>
          <li class="nav-item">
            <RouterLink class="nav-link" to="/pricing">Pricing</RouterLink>
          </li>
        </ul>

        <!-- Center logo -->
        <h1 class="h4 m-0 nav-center">nometa.id</h1>

        <!-- Right side: desktop nav or mobile hamburger -->
        <div class="d-flex align-items-center">
          <!-- Desktop right nav -->
          <ul v-if="windowWidth >= MOBILE_BREAKPOINT" class="nav nav-right">
            <li class="nav-item">
              <RouterLink class="nav-link" to="/login">Login</RouterLink>
            </li>
            <li class="nav-item button">
              <RouterLink class="nav-link" to="/signup">Sign Up</RouterLink>
            </li>
          </ul>

          <!-- Mobile hamburger -->
          <button 
            v-if="windowWidth < MOBILE_BREAKPOINT" 
            class="menu-btn ms-3" 
            @click="toggleMenu">
            <span :class="{ open: isOpen }"></span>
            <span :class="{ open: isOpen }"></span>
            <span :class="{ open: isOpen }"></span>
          </button>
        </div>
      </div>


      <!-- Mobile menu -->
      <transition name="slide-right">
        <ul v-if="isOpen && windowWidth < MOBILE_BREAKPOINT" class="mobile-menu nav flex-column">
          <li class="nav-item">
            <RouterLink class="nav-link" to="/contact" @click="toggleMenu">Contact</RouterLink>
          </li>
          <li class="nav-item">
            <RouterLink class="nav-link" to="/pricing" @click="toggleMenu">Pricing</RouterLink>
          </li>
          <li class="nav-item">
            <RouterLink class="nav-link" to="/login" @click="toggleMenu">Login</RouterLink>
          </li>
          <li class="nav-item button">
            <RouterLink class="nav-link" to="/signup" @click="toggleMenu">Sign Up</RouterLink>
          </li>
        </ul>
      </transition>
    </nav>
  </header>
</template>


<style scoped>
/* Hamburger icon */
.menu-btn {
  position: relative; /* or fixed if you want it to stay while scrolling */
  z-index: 1000;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  width: 28px;
  height: 22px;
  background: none;
  border: none;
  cursor: pointer;
  padding: 0;
}
.menu-btn span {
  display: block;
  height: 3px;
  width: 27px; 
  background-color: var(--color-text);
  border-radius: 2px;
  transition: all 0.3s ease;
  transform-origin: 0; /* <-- important */
}

.menu-btn span.open:nth-child(1) {
  transform: rotate(45deg);
}

.menu-btn span.open:nth-child(2) {
  opacity: 0;
}

.menu-btn span.open:nth-child(3) {
  transform: rotate(-45deg);
}

/* Mobile menu sliding from right */
.mobile-menu {
  position: fixed;
  top: 0;
  right: 0; /* slide from right now */
  width: 80%;
  height: 100vh;
  background-color: var(--color-background);
  z-index: 999;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 1.5rem;
}
.mobile-menu .nav-link {
  display: block;
  font-size: 1.5rem;
  font-weight: bold;
  color: var(--color-text);
  text-align: center;
}
.mobile-menu .nav-item.button .nav-link {
  border: 2px solid var(--vt-c-green);
  border-radius: 20px;
  width: 150px;
  padding: 0.5rem 1.5rem;
  color: white;
}
.mobile-menu .nav-item.button .nav-link:hover {
  background-color: var(--vt-c-green);
  color: white;
}

/* Slide from right animation */
.slide-right-enter-from {
  transform: translateX(100%);
  opacity: 0;
}
.slide-right-enter-to {
  transform: translateX(0);
  opacity: 1;
}
.slide-right-leave-from {
  transform: translateX(0);
  opacity: 1;
}
.slide-right-leave-to {
  transform: translateX(100%);
  opacity: 0;
}
.slide-right-enter-active,
.slide-right-leave-active {
  transition: all 0.3s ease;
}

/* Desktop nav styling */
.nav-link {
  color: var(--color-text);
  font-weight: bold;
}
.nav-link:hover {
  color: var(--vt-c-green);
}
.nav-item.button .nav-link {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  text-align: center;
  width: 7rem;
  padding: 0.25rem 0.75rem;
  border-radius: 20px;
  border: 2px solid var(--vt-c-green);
  transition: all 0.2s;
  white-space: nowrap;          /* prevent wrapping */
}
.nav-item.button .nav-link:hover {
  background-color: var(--vt-c-green);
  color: white;
}

header h1 {
  color: white;
  font-size: 40px;
}
</style>
