// Domain model for events. Shared across features, so it must not import
// anything from features/, services/ or app/.

export const EVENT_CATEGORIES = ['music', 'sport', 'tech', 'food', 'art'] as const

export type EventCategory = (typeof EVENT_CATEGORIES)[number]

export interface EventImage {
  id: string
  url: string
  alt: string
}

// Calling it SsEvent to avoid confusion with the built-in JS/browser Event type.
export interface SsEvent {
  id: string
  title: string
  description: string
  category: EventCategory
  /** ISO 8601 date-time, e.g. "2026-10-17T19:30:00Z" */
  startsAt: string
  venue: string
  city: string
  /** Shown in the event list. */
  coverImage: EventImage
  /** Additional images shown on the detail view. */
  images: EventImage[]
}
