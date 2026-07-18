import { ref, computed, watch, onBeforeUnmount, type Ref } from 'vue'

const STORAGE_PREFIX = 'password-reset-last-request:'
const COOLDOWN_SECONDS = 5 * 60 // keep in sync with backend RESEND_COOLDOWN_MINUTES

function storageKey(email: string): string {
  return `${STORAGE_PREFIX}${email.trim().toLowerCase()}`
}

function readStoredTimestamp(email: string): number | null {
  if (!email) return null
  const raw = localStorage.getItem(storageKey(email))
  return raw ? Number(raw) : null
}

export function usePasswordResetCooldown(email: Ref<string>) {
  const now = ref(Date.now())
  const lastRequestAt = ref<number | null>(readStoredTimestamp(email.value))

  // re-read from storage whenever the typed email changes
  watch(email, (newEmail) => {
    lastRequestAt.value = readStoredTimestamp(newEmail)
  })

  const intervalId = setInterval(() => {
    now.value = Date.now()
  }, 1000)

  onBeforeUnmount(() => clearInterval(intervalId))

  const remainingSeconds = computed(() => {
    if (!lastRequestAt.value) return 0
    const elapsed = Math.floor((now.value - lastRequestAt.value) / 1000)
    return Math.max(0, COOLDOWN_SECONDS - elapsed)
  })

  const isInCooldown = computed(() => remainingSeconds.value > 0)

  const formattedRemaining = computed(() => {
    const total = remainingSeconds.value
    const minutes = Math.floor(total / 60)
    const seconds = total % 60
    return `${minutes}:${seconds.toString().padStart(2, '0')}`
  })

  function startCooldown(): void {
    if (!email.value) return
    const timestamp = Date.now()
    localStorage.setItem(storageKey(email.value), timestamp.toString())
    lastRequestAt.value = timestamp
  }

  return { remainingSeconds, isInCooldown, formattedRemaining, startCooldown }
}
