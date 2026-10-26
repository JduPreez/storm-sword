import type { EventType } from '../../models'

// Types only the events feature cares about. Shared domain types live in src/models.

export interface EventFilter {
  search?: string
  eventType?: EventType
  countryCode?: string
}
