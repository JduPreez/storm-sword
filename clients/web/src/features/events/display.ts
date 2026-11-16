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
export const COUNTRY_CODES = [
  'AD', // Andorra
  'AR', // Argentina
  'AU', // Australia
  'AT', // Austria
  'BR', // Brazil
  'CA', // Canada
  'CL', // Chile
  'CN', // China
  'HR', // Croatia
  'EC', // Ecuador
  'FR', // France
  'DE', // Germany
  'HK', // Hong Kong
  'ID', // Indonesia
  'IT', // Italy
  'JP', // Japan
  'LV', // Latvia
  'MY', // Malaysia
  'MU', // Mauritius
  'MX', // Mexico
  'NZ', // New Zealand
  'OM', // Oman
  'PL', // Poland
  'PT', // Portugal
  'RO', // Romania
  'SI', // Slovenia
  'ZA', // South Africa
  'KR', // South Korea
  'ES', // Spain
  'SE', // Sweden
  'CH', // Switzerland
  'TW', // Taiwan
  'TH', // Thailand
  'TR', // Turkey
  'GB', // United Kingdom
  'US', // United States
  'VN', // Vietnam
] as const

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
