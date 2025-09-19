<template>
  <div class="narrative-table-container">
    <v-data-table
      :headers="headers"
      :items="narratives"
      class="narrative-table"
      hide-default-footer
      item-value="id"
    >
      <template #item.thumbnail="{ item }">
        <div class="thumbnail-cell">
          <v-img
            alt="Narrative thumbnail"
            aspect-ratio="16/9"
            class="thumbnail-image"
            cover
            :src="item.thumbnail"
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
            :color="item.authorColor"
            size="small"
            variant="tonal"
          >
            <v-avatar class="author-avatar" start>
              <v-img alt="Author avatar" :src="item.authorAvatar" />
            </v-avatar>
            {{ item.authorName }}
          </v-chip>
        </div>
      </template>

      <template #item.duration="{ item }">
        <div class="duration-cell">
          <span class="muted-text">{{ item.duration }}</span>
        </div>
      </template>

      <template #item.number="{ item }">
        <div class="number-cell">
          <span class="muted-text">{{ item.number }}</span>
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
  </div>
</template>

<script setup lang="ts">
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

  const narratives = [
    {
      id: 1,
      thumbnail: 'https://picsum.photos/200/120?random=1',
      title: 'The evil scientist and his magical chair',
      authorName: 'Kostas',
      authorAvatar: 'https://i.pravatar.cc/40?img=1',
      authorColor: 'blue-grey',
      duration: '02:14',
      number: '1st',
    },
    {
      id: 2,
      thumbnail: 'https://picsum.photos/200/120?random=2',
      title: 'Adventures in the Digital Realm',
      authorName: 'Maria',
      authorAvatar: 'https://i.pravatar.cc/40?img=2',
      authorColor: 'purple',
      duration: '05:32',
      number: '2nd',
    },
    {
      id: 3,
      thumbnail: 'https://picsum.photos/200/120?random=3',
      title: 'The Mystery of the Lost Algorithm',
      authorName: 'Alex',
      authorAvatar: 'https://i.pravatar.cc/40?img=3',
      authorColor: 'teal',
      duration: '03:45',
      number: '3rd',
    },
  ]

  function participate (id: number) {
    console.log(`Participating in narrative ${id}`)
    // TODO: Add participation logic here
  }
</script>

<style scoped lang="sass">
.narrative-table-container
  width: 100%
  margin: 0
  padding: 0

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

  // Dark theme alternative
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
</style>
