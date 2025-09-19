<template>
  <div class="playful-header">
    <div class="header-content">
      <div class="app-title">
        <span class="emoji-decoration">📚</span>
        <span class="app-name">Narratasync</span>
      </div>

      <div class="header-actions">
        <a
          class="fun-link github-link"
          href="https://github.com/kongr45gpen/narratasync"
          rel="noopener noreferrer"
          target="_blank"
          title="View on GitHub"
        >
          <svg class="github-icon" fill="currentColor" viewBox="0 0 24 24">
            <path d="M12 0C5.374 0 0 5.373 0 12 0 17.302 3.438 21.8 8.207 23.387c.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0112 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z" />
          </svg>
          <span class="link-text">GitHub</span>
        </a>

        <div class="dropdown-container">
          <button class="fun-link dropdown-trigger" @click="toggleDropdown">
            <span class="link-text">More</span>
            <span class="dropdown-arrow" :class="{ open: isDropdownOpen }">▼</span>
          </button>

          <div v-show="isDropdownOpen" class="fun-dropdown">
            <a
              v-for="item in menuItems"
              :key="item.title"
              class="dropdown-item"
              :href="item.href"
              rel="noopener noreferrer"
              target="_blank"
            >
              <svg class="item-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path :d="item.iconPath" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" />
              </svg>
              <span class="item-text">{{ item.title }}</span>
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue'

  const isDropdownOpen = ref(false)

  function toggleDropdown () {
    isDropdownOpen.value = !isDropdownOpen.value
  }

  const menuItems = [
    {
      title: 'Documentation',
      iconPath: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253',
      href: 'https://github.com/kongr45gpen/narratasync#readme',
    },
    {
      title: 'Issues',
      iconPath: 'M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z',
      href: 'https://github.com/kongr45gpen/narratasync/issues',
    },
    {
      title: 'Releases',
      iconPath: 'M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z',
      href: 'https://github.com/kongr45gpen/narratasync/releases',
    },
    {
      title: 'License',
      iconPath: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z',
      href: 'https://github.com/kongr45gpen/narratasync/blob/main/LICENSE',
    },
  ]
</script>

<style scoped lang="sass">
  .playful-header
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%)
    padding: 16px 24px
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15)
    position: relative

  .header-content
    display: flex
    justify-content: space-between
    align-items: center
    max-width: 1200px
    margin: 0 auto
    position: relative
    z-index: 1

  .app-title
    display: flex
    align-items: center
    gap: 12px

  .emoji-decoration
    font-size: 1.5rem
    filter: drop-shadow(0 2px 4px rgba(0,0,0,0.2))

  .app-name
    font-family: 'Zalando Sans Expanded', 'Helvetica Neue', Arial, sans-serif
    font-weight: 400
    font-size: 1.8rem
    color: #ffffff
    text-shadow: 0 2px 4px rgba(0,0,0,0.3)
    letter-spacing: 1px

  .header-actions
    display: flex
    align-items: center
    gap: 16px

  .fun-link
    display: flex
    align-items: center
    gap: 8px
    padding: 10px 16px
    background: rgba(255, 255, 255, 0.2)
    border: 2px solid rgba(255, 255, 255, 0.3)
    border-radius: 25px
    color: #ffffff
    text-decoration: none
    font-weight: 500
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1)
    backdrop-filter: blur(10px)
    cursor: pointer

    &:hover
      background: rgba(255, 255, 255, 0.3)
      border-color: rgba(255, 255, 255, 0.5)
      transform: translateY(-2px) scale(1.05)
      box-shadow: 0 8px 25px rgba(0, 0, 0, 0.2)

  .link-emoji, .item-icon
    width: 1.2rem
    height: 1.2rem
    filter: drop-shadow(0 1px 2px rgba(0,0,0,0.2))

  .github-icon
    width: 1.2rem
    height: 1.2rem
    filter: drop-shadow(0 1px 2px rgba(0,0,0,0.2))

  .link-text, .item-text
    font-size: 0.9rem

  .dropdown-container
    position: relative

  .dropdown-arrow
    transition: transform 0.3s ease
    font-size: 0.8rem
    margin-left: 4px

    &.open
      transform: rotate(180deg)

  .fun-dropdown
    position: absolute
    top: calc(100% + 8px)
    right: 0
    background: rgba(255, 255, 255, 0.95)
    backdrop-filter: blur(15px)
    border-radius: 16px
    padding: 8px
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2)
    border: 1px solid rgba(255, 255, 255, 0.3)
    min-width: 180px
    z-index: 1000

  .dropdown-item
    display: flex
    align-items: center
    gap: 12px
    padding: 12px 16px
    color: #333
    text-decoration: none
    border-radius: 12px
    transition: all 0.2s ease
    font-weight: 500

    &:hover
      background: rgba(102, 126, 234, 0.1)
      transform: translateX(4px)

  @keyframes bounce
    0%, 20%, 50%, 80%, 100%
      transform: translateY(0)
    40%
      transform: translateY(-8px)
    60%
      transform: translateY(-4px)

  @keyframes float
    0%, 100%
      transform: translate(0, 0) rotate(0deg)
    33%
      transform: translate(30px, -30px) rotate(120deg)
    66%
      transform: translate(-20px, 20px) rotate(240deg)
</style>
