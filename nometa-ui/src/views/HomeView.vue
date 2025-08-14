<script setup lang="ts">
import PageHeader from '@/components/PageHeader.vue';
import heroVideo from '@/assets/videos/hero-background.mp4';
</script>

<template>
  <PageHeader class="blur-header" data-aos="fade-down" data-aos-duration="500" />
  
  <!-- Hero section -->
  <div class="hero d-flex flex-column justify-content-center align-items-center text-center pb-7">
    <video :src="heroVideo" autoplay muted loop class="hero-video">
      <!-- <source src="@/assets/videos/hero-background.mp4" type="video/mp4" /> -->
      Your browser does not support the video tag.
    </video>
    
    <div class="opener" data-aos="zoom-out" data-aos-duration="500">
      <h2 class="mb-3">Show who you are</h2>
      <p class="lead">nometa.id is your way to show the world your digital identity</p>
    </div>

    <div class="claim mt-5" data-aos="zoom-in" data-aos-delay="100" data-aos-duration="500">
      <p class="mb-3">Claim your unique address now!</p>
      <div class="d-flex gap-2 mt-2 claim-input">
          <div class="custom-input d-flex align-items-center">
              <span class="prefix">nometa.id/</span>
              <input type="text" class="new-username" placeholder="username" autocomplete="off"/>
          </div>
          <button class="claim-btn">Claim</button>
      </div>
    </div>
  </div>

  <!-- Features section -->
  <div id="features" class="d-flex flex-column justify-content-center align-items-center claim text-center">
    <h1 class="mt-5 mb-6">Features</h1>
    
    <div class="features-container d-flex gap-4 flex-wrap justify-content-center">
      <div class="feature-box" id="pink" data-aos="fade-down">
        <img src="@/assets/images/customize.png" alt="Customize" class="feature-icon">
        <h3>Customization</h3>
        <p>Customize your personal bio with colors, music, videos, and more!</p>
      </div>

      <div class="feature-box" id="green" data-aos="fade-down" data-aos-delay="100">
        <img src="@/assets/images/analytics.png" alt="Analytics" class="feature-icon">
        <h3>Analytics</h3>
        <p>Get insights into your bio's visits, referrers, and even geographical reach.</p>
      </div>

      <div class="feature-box" id="pink" data-aos="fade-down" data-aos-delay="200">
        <img src="@/assets/images/socials.png" alt="Socials" class="feature-icon">
        <h3>Social Links</h3>
        <p>Add social links to your profile so everyone knows where to find you.</p>
      </div>

      <div class="feature-box" id="green" data-aos="fade-down" data-aos-delay="300">
        <img src="@/assets/images/lock.png" alt="Lock" class="feature-icon">
        <h3>Secure</h3>
        <p>Add data is stored securely with AES 256-bit encryption where needed.</p>
      </div>

      <div class="feature-box" id="pink" data-aos="fade-down" data-aos-delay="400">
        <img src="@/assets/images/reliable.png" alt="Reliable" class="feature-icon">
        <h3>Reliable</h3>
        <p>We use reliable servers with high up-time so that you can always access your bio.</p>
      </div>
    </div>
  </div>

  <!-- Pricing section -->
  <div id="features" class="d-flex flex-column justify-content-center align-items-center claim text-center pt-5">
    <h1 class="mt-5 mb-6">Pricing</h1>
    
    <div class="features-container d-flex gap-4 flex-wrap justify-content-center">
      <div class="feature-box" data-aos="zoom-in">
        <h3>Free</h3>
        <p>$0<span class="alt">/Lifetime</span></p>
        <ul>
          <li>Basic customization</li>
          <li>Social links</li>
          <li>Analytics</li>
        </ul>
      </div>

      <div class="feature-box" id="green" data-aos="zoom-out">
        <h3>Premium</h3>
        <p>$5<span class="alt">/Month</span></p>
        <ul>
          <li>Premium badge</li>
          <li>Custom social icons</li>
          <li>Custom fonts</li>
          <li>Profile effects</li>
        </ul>
      </div>

      <div class="feature-box" id="pink" data-aos="zoom-in">
        <h3>VIP</h3>
        <p>Invite Only</p>
        <p class="sub">Must have a premium plan</p>
        <ul>
          <li>VIP badge</li>
          <li>Gain access to VIP areas and communities</li>
          <li>Test new features early</li>
          <li>Access to future private forum</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<style scoped>
#features {
  background-color: var(--color-background);
  color: var(--color-text);
}

.feature-box {
  width: 22rem;
  height: 25rem;
  background-color: var(--color-background-soft);
  border: 2px solid var(--color-background-soft);
  border-radius: 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-start;
  padding: 1rem;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

/* Only apply hover AFTER AOS animation is done */
.feature-box.aos-animate:hover {
  transform: translateY(-10px) !important;
  box-shadow: 0 12px 20px rgba(0, 0, 0, 0.3) !important;
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.feature-box#green {
  border: 2px solid var(--vt-c-green);
}
.feature-box#pink {
  border: 2px solid var(--vt-c-pink);
}
.feature-box li {
  text-align: left;
}


.feature-box p {
  padding-top: 1rem;
}

.feature-icon {
  width: 180px;          
  height: 180px;         
  margin-bottom: 2.5rem;
  object-fit: contain;
}

.hero {
  color: var(--color-text);
  position: relative;
  min-height: 100vh;
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  background-color: var(--vt-c-black);
  overflow: hidden;
}

.hero-video {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 100%;
  height: 100%;
  object-fit: cover;
  transform: translate(-50%, -50%);
  filter: blur(8px) brightness(0.6); /* blur and darken */
  z-index: 0;
}

.hero > *:not(.hero-video) {
  position: relative;
  z-index: 1;
}

.blur-header {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  padding: 1rem 2rem;
  background-color: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  z-index: 1000;
  transition: background-color 0.3s ease;
}

.opener h2 {
  font-style: bold;
  font-size: 36px;
}

.opener p {
  font-size: 20px;;
}

.mt-6 {
  margin-top: 7rem;
}

.mb-6 {
  margin-bottom: 7rem;
}

.pb-6 {
  padding-bottom: 10rem;
}

.input-group-text {
    border: 1px solid white !important;
    border-radius: 0.7rem 0 0 0.7rem;
    background-color: var(--vt-c-black);
    color: white;
}

/* Base styling for the combined input */
.claim-input .custom-input {
  display: flex;
  border: 2px solid white;
  border-radius: 0.7rem;
  overflow: hidden;
  background-color: var(--vt-c-black);
  height: 2.5rem;
  width: 15rem;
  box-sizing: border-box;
  transition: border 0.2s, box-shadow 0.2s; 
}

/* Hover effect on the whole input */
.claim-input .custom-input:hover {
  border-color: var(--vt-c-green);
  box-shadow: 0 0 0 0.1rem var(--vt-c-green);
}

/* Focus effect when input is selected */
.claim-input .custom-input:has(input:focus) {
  border-color: var(--vt-c-green);
  box-shadow: 0 0 0 0.1rem var(--vt-c-green);
}

/* Prefix styling remains the same */
.claim-input .prefix {
  display: flex;
  align-items: center;
  padding-left: 0.5rem;
  padding-right: 0.1rem;
  background-color: var(--vt-c-black);
  color: white;
  user-select: none;
  line-height: 1;
}

/* Input styling remains the same */
.claim-input .new-username {
  border: none;
  outline: none;
  padding-left: 0;
  padding-right: 0.5rem;
  background-color: transparent;
  color: white;
  flex: 1;
  min-width: 0;
  line-height: 1;
}

.claim-btn {
    color: var(--color-text);
    background-color: transparent;
    border-radius: 0.7rem;
    width: 6rem;
    border: 2px solid var(--vt-c-green);
    transition: all 0.2s;
}
.claim-btn:hover {
    background-color: var(--vt-c-green);
}

.divide-line {
    background-color: white;
    opacity: 1;
    width: 90%;
    margin: 0 auto;
}

</style>
