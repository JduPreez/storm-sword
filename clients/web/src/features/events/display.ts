import type { EventCategory } from '../../models'

// UI helpers for presenting events. Kept out of src/models because they are
// presentation concerns, not part of the domain model.

export const CATEGORY_LABELS: Record<EventCategory, string> = {
  music: 'Music',
  sport: 'Sport',
  tech: 'Tech',
  food: 'Food & Drink',
  art: 'Art',
}

export const CATEGORY_COLORS: Record<EventCategory, string> = {
  music: 'magenta',
  sport: 'green',
  tech: 'blue',
  food: 'orange',
  art: 'purple',
}

const dateTimeFormat = new Intl.DateTimeFormat(undefined, {
  weekday: 'short',
  day: 'numeric',
  month: 'short',
  hour: '2-digit',
  minute: '2-digit',
})

const monthFormat = new Intl.DateTimeFormat(undefined, { month: 'long', year: 'numeric' })

export const formatEventDate = (iso: string) => dateTimeFormat.format(new Date(iso))

export const formatEventMonth = (iso: string) => monthFormat.format(new Date(iso))
