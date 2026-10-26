import type { EventType, DateTime } from '../../models'

// UI helpers for presenting events. Kept out of src/models because they are
// presentation concerns, not part of the domain model.

export const EVENT_TYPE_LABELS: Record<EventType, string> = {
  TRAIL_RUN: 'Trail Run',
  MOUNTAIN_BIKE: 'Mountain Bike',
  EXHIBITION: 'Exhibition',
  HIKE: 'Hike',
}

export const EVENT_TYPE_COLORS: Record<EventType, string> = {
  TRAIL_RUN: 'magenta',
  MOUNTAIN_BIKE: 'green',
  EXHIBITION: 'blue',
  HIKE: 'orange',
}

// Countries offered in the filter. The API lists events per country, so add a
// code here when events start appearing somewhere new.
export const COUNTRY_CODES = ['DE', 'NL', 'PT'] as const

const regionNames = new Intl.DisplayNames(undefined, { type: 'region' })

export const formatCountry = (countryCode: string) => regionNames.of(countryCode) ?? countryCode

const dateTimeFormat = new Intl.DateTimeFormat(undefined, {
  weekday: 'short',
  day: 'numeric',
  month: 'short',
  hour: '2-digit',
  minute: '2-digit',
})

const monthFormat = new Intl.DateTimeFormat(undefined, { month: 'long', year: 'numeric' })

export const formatEventDate = (iso: DateTime | undefined) =>
  iso ? dateTimeFormat.format(new Date(iso)) : '';

export const formatEventMonth = (iso: DateTime | undefined) =>
  iso ? monthFormat.format(new Date(iso)) : '';
