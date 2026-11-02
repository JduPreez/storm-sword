// Domain model for events. Shared across features, so it must not import
// anything from features/, services/ or app/.
export const EVENT_TYPES = ['TRAIL_RUN', 'MOUNTAIN_BIKE', 'EXHIBITION', 'HIKE'] as const

export type EventType = (typeof EVENT_TYPES)[number]

export interface EventImage {
  id?: string
  url?: string
  alt?: string
}

export interface Address {
  countryCode: string,
  addressLine1?: string,
  addressLine2?: string,
  cityTownLocality?: string,
  sublocality?: string, // Suburb, Borough, Barrio
  administrativeArea: string, // State, Province, Prefecture, County
  postalCode?: string, // ZIP code, Postcode, PIN code
  latitude?: number,
  longitude?: number,
  description?: string
}

export type DateTime = string | number | null;

// Calling it SsEvent to avoid confusion with the built-in JS/browser Event type.
export interface SsEvent {
  id?: string
  ns?: string
  eventType: EventType
  name: string
  startDate?: DateTime
  endDate?: DateTime
  distanceMin?: number
  distanceMax?: number
  address: Address
  description?: string
  /** Shown in the event list. */
  coverImage?: EventImage
  /** Additional images shown on the detail view. */
  images?: EventImage[]
}
