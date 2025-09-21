<template>
  <div class="preview-page">
    <div class="preview-content">
      <h1 class="preview-title">Page {{ id }}</h1>
      <div class="preview-image-container">
        <img
          :alt="`Preview for ${id}`"
          class="preview-image"
          src="https://picsum.photos/1200/600?random=6"
        >
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue'
  import { useRoute } from 'vue-router'

  const route = useRoute()
  const id = computed(() => {
    const params = route.params as { id?: string | string[] }
    const routeId = params.id
    return typeof routeId === 'string' ? routeId : (Array.isArray(routeId) ? routeId[0] : 'unknown')
  })
</script>

<style scoped lang="sass">
.preview-page
  width: 100%
  margin: 0
  padding: 0

.preview-content
  padding: 60px 24px
  text-align: center
  max-width: 1200px
  margin: 0 auto

.preview-title
  font-family: var(--fancy-font, sans-serif)
  font-weight: 400
  font-size: 2rem
  color: rgba(var(--v-theme-on-surface), 0.87)
  margin: 0 0 2rem 0
  letter-spacing: 0.5px

  // Dark theme variant
  .v-theme--dark &
    color: rgba(var(--v-theme-on-surface), 0.95)

.preview-image-container
  width: 100%
  margin: 0 auto
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1)

  .v-theme--dark &
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3)

.preview-image
  width: 100%
  height: auto
  display: block
  object-fit: cover
</style>
