<script setup lang="ts">
import TodayBirthdayCard from '@/components/member/TodayBirthdayCard.vue'
</script>

<template>
  <div class="p-6 space-y-6">
    <div>
      <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">Dashboard</h1>
      <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">Welcome back</p>
    </div>

    <TodayBirthdayCard />
  </div>
</template>
