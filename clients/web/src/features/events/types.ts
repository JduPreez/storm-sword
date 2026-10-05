import type { EventCategory } from '../../models'

// Types only the events feature cares about. Shared domain types live in src/models.

export interface EventFilter {
  search?: string
  category?: EventCategory
}
