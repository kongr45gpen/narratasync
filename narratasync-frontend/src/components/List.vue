<template>
  <div class="narrative-table-container">
    <!-- Welcome section -->
    <div class="welcome-section">
      <h1 class="welcome-heading">Welcome to Narratasync!</h1>
    </div>

    <v-data-table
      class="narrative-table"
      :headers="headers"
      hide-default-footer
      item-value="id"
      :items="narratives"
      :loading="isLoading"
      loading-text="Loading narratives..."
    >
      <!-- Error and empty states -->
      <template #no-data>
        <div v-if="error && !isLoading" class="error-state">
          <v-icon color="error" size="48">mdi-alert-circle</v-icon>
          <h3 class="error-title">Failed to load narratives</h3>
          <p class="error-message">{{ error }}</p>
          <v-btn
            color="primary"
            variant="outlined"
            @click="fetchNarratives"
          >
            Retry
          </v-btn>
        </div>

        <div v-else-if="!error && !isLoading && narratives.length === 0" class="empty-state">
          <v-icon color="grey" size="48">mdi-book-open</v-icon>
          <h3 class="empty-title">No narratives found</h3>
          <p class="empty-message">Check back later for new narratives!</p>
        </div>
      </template>
      <template #item.thumbnail="{ item }">
        <div class="thumbnail-cell">
          <v-img
            alt="Narrative thumbnail"
            aspect-ratio="16/9"
            class="thumbnail-image"
            cover
            :src="item.thumbnail || 'https://picsum.photos/200/120?random=default'"
          />
        </div>
      </template>

      <template #item.title="{ item }">
        <div class="title-cell">
          <span class="title-text">{{ item.title }}</span>
        </div>
      </template>

      <template #item.author="{ item }">
        <div class="author-cell">
          <v-chip
            class="author-chip"
            :color="item.authorColor || 'grey'"
            size="small"
            variant="tonal"
          >
            <v-avatar class="author-avatar" start>
              <v-img alt="Author avatar" :src="item.authorAvatar || 'https://i.pravatar.cc/40?img=default'" />
            </v-avatar>
            {{ item.authorName || item.author || 'Unknown' }}
          </v-chip>
        </div>
      </template>

      <template #item.duration="{ item }">
        <div class="duration-cell">
          <span class="muted-text">{{ item.duration || 'N/A' }}</span>
        </div>
      </template>

      <template #item.number="{ item }">
        <div class="number-cell">
          <span class="muted-text">{{ item.number || 'N/A' }}</span>
        </div>
      </template>

      <template #item.actions="{ item }">
        <div class="actions-cell">
          <v-btn
            class="participate-btn"
            color="primary"
            prepend-icon="mdi-plus"
            rounded="xl"
            size="default"
            variant="elevated"
            @click="participate(item.id)"
          >
            Participate!
          </v-btn>

          <v-btn
            class="view-btn"
            color="grey"
            disabled
            prepend-icon="mdi-eye"
            rounded="xl"
            size="small"
            variant="outlined"
          >
            View
          </v-btn>
        </div>
      </template>
    </v-data-table>

    <!-- Decorative pattern -->
    <div class="table-footer-pattern">
      <div class="pattern-container">
        <div class="pattern-dots">
          <div v-for="i in 24" :key="i" class="dot" :style="{ animationDelay: `${i * 0.1}s` }" />
        </div>
        <div class="pattern-waves">
          <svg class="wave-svg" viewBox="0 0 120 20" xmlns="http://www.w3.org/2000/svg">
            <path class="wave-path" d="M0,10 Q15,0 30,10 T60,10 T90,10 T120,10" />
            <path class="wave-path wave-path-2" d="M0,15 Q15,5 30,15 T60,15 T90,15 T120,15" />
          </svg>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { onMounted, ref } from 'vue'

  interface Narrative {
    id: string
    title: string
    thumbnail: string | null
    author: string | null
    authorName?: string
    authorAvatar?: string
    authorColor?: string
    duration?: string
    number?: string
  }

  const headers = [
    {
      title: 'Thumbnail',
      key: 'thumbnail',
      sortable: false,
      width: '120px',
    },
    {
      title: 'Title',
      key: 'title',
      sortable: true,
    },
    {
      title: 'Who',
      key: 'author',
      sortable: true,
      width: '200px',
    },
    {
      title: 'Duration',
      key: 'duration',
      sortable: true,
      width: '100px',
    },
    {
      title: 'Number',
      key: 'number',
      sortable: true,
      width: '100px',
    },
    {
      title: 'Actions',
      key: 'actions',
      sortable: false,
      width: '250px',
    },
  ]

  const narratives = ref<Narrative[]>([])
  const isLoading = ref(true)
  const error = ref<string | null>(null)

  async function fetchNarratives () {
    try {
      isLoading.value = true
      error.value = null

      const response = await fetch('http://localhost:3002/')

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }

      const data = await response.json()
      narratives.value = data
    } catch (error_) {
      error.value = error_ instanceof Error ? error_.message : 'Failed to fetch narratives'
      console.error('Error fetching narratives:', error_)
    } finally {
      isLoading.value = false
    }
  }

  onMounted(() => {
    fetchNarratives()
  })

  function participate (id: string) {
    console.log(`Participating in narrative ${id}`)
    // TODO: Add participation logic here
  }
</script>

<style scoped lang="sass">
.narrative-table-container
  width: 100%
  margin: 0
  padding: 0

.welcome-section
  padding: 60px 24px 40px
  text-align: center

.welcome-heading
  font-family: 'Zalando Sans Expanded', 'Helvetica Neue', Arial, sans-serif
  font-weight: 400
  font-size: 2.5rem
  color: rgba(var(--v-theme-on-surface), 0.87)
  margin: 0
  letter-spacing: 1px
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1)

  // Dark theme variant
  .v-theme--dark &
    color: rgba(var(--v-theme-on-surface), 0.95)
    text-shadow: 0 2px 4px rgba(255, 255, 255, 0.05)

.narrative-table
  width: 100%
  max-width: 1400px;
  margin: auto;

  :deep(.v-data-table__wrapper)
    border-radius: 0

  :deep(.v-data-table-header)
    background-color: rgba(var(--v-theme-surface-variant), 0.5)

  :deep(.v-data-table-header__content)
    font-weight: 600
    padding: 16px 24px

  :deep(.v-data-table__td)
    padding: 16px 24px
    border-bottom: 1px solid rgba(var(--v-theme-outline), 0.12)

.thumbnail-cell
  padding: 8px 0

.thumbnail-image
  width: 80px
  height: 45px
  border-radius: 8px
  overflow: hidden
  margin: auto

.title-cell
  min-width: 200px

.title-text
  font-weight: 500
  font-size: 1rem
  line-height: 1.4

.author-cell
  display: flex
  align-items: center

.author-chip
  font-weight: 500

.author-avatar
  width: 24px !important
  height: 24px !important

.duration-cell, .number-cell
  text-align: center

.muted-text
  color: rgba(var(--v-theme-on-surface), 0.6)
  font-size: 0.875rem
  font-weight: 400

.actions-cell
  display: flex
  align-items: center
  gap: 12px
  justify-content: flex-start

.participate-btn
  background: linear-gradient(135deg, #00D270 0%, #1391AD 100%) !important
  color: white !important
  font-weight: 600
  text-transform: none
  letter-spacing: 0.5px
  box-shadow: 0 4px 15px rgba(76, 175, 80, 0.4)
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1)

  &:hover
    transform: translateY(-2px)
    box-shadow: 0 6px 20px rgba(76, 175, 80, 0.6)

  :deep(.v-icon)
    color: rgba(255, 255, 255, 0.9)
    font-size: 1.1rem

  .v-theme--dark &
    background: linear-gradient(135deg, #0F7F3E 0%, #1B4B68 100%) !important
    box-shadow: 0 4px 15px rgba(15, 127, 62, 0.4)

    &:hover
      box-shadow: 0 6px 20px rgba(15, 127, 62, 0.6)

.view-btn
  font-size: 0.875rem
  text-transform: none
  opacity: 0.6

  :deep(.v-icon)
    color: rgba(var(--v-theme-on-surface), 0.5)
    font-size: 1rem

// Error and empty states
.error-state, .empty-state
  text-align: center
  padding: 40px 20px

.error-title, .empty-title
  margin: 16px 0 8px
  font-size: 1.25rem
  font-weight: 500

.error-message, .empty-message
  margin: 0 0 24px
  color: rgba(var(--v-theme-on-surface), 0.6)
  font-size: 0.875rem

// Decorative pattern at the end of table
.table-footer-pattern
  margin-top: 40px
  padding: 30px 0
  position: relative
  overflow: hidden

.pattern-container
  max-width: 1400px
  margin: 0 auto
  position: relative
  height: 60px

.pattern-dots
  display: flex
  justify-content: center
  align-items: center
  gap: 8px
  margin-bottom: 20px

.dot
  width: 4px
  height: 4px
  border-radius: 50%
  background: rgba(var(--v-theme-primary), 0.3)
  animation: pulse-dot 2s ease-in-out infinite
  opacity: 0.6

  // Dark theme variant
  .v-theme--dark &
    background: rgba(var(--v-theme-primary), 0.2)

.pattern-waves
  position: absolute
  bottom: 0
  left: 50%
  transform: translateX(-50%)
  width: 100%
  max-width: 400px

.wave-svg
  width: 100%
  height: 20px
  opacity: 0.4

.wave-path
  fill: none
  stroke: rgba(var(--v-theme-primary), 0.4)
  stroke-width: 1
  stroke-linecap: round
  animation: wave-flow 4s ease-in-out infinite

.wave-path-2
  stroke: rgba(var(--v-theme-secondary), 0.3)
  animation-delay: 1s
  animation-duration: 5s

// Animations
@keyframes pulse-dot
  0%, 100%
    opacity: 0.3
    transform: scale(1)
  50%
    opacity: 0.8
    transform: scale(1.2)

@keyframes wave-flow
  0%, 100%
    stroke-dasharray: 0 100
    stroke-dashoffset: 0
  50%
    stroke-dasharray: 50 50
    stroke-dashoffset: -25
</style>
