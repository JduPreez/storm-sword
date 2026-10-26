import type { EventImage, SsEvent } from '../../models'
import type { ListEventsRequest } from '../../services/eventsApi'

// Stand-in for the backend until it has real data. Events are in the raw shape
// the API returns (epoch-ms dates, no address description); the mock API runs
// them through the same transform as the real one.

const image = (id: string, alt: string): EventImage => ({
  id,
  alt,
  // picsum.photos returns a stable random photo per seed.
  url: `https://picsum.photos/seed/${id}/1200/675`,
})

export const mockEvents: SsEvent[] = [
  {
    id: 'q7m2xk9v4t8b1n6c3p5r0w2z',
    eventType: 'TRAIL_RUN',
    name: 'Sintra Trail Series: Round 3',
    description: 'A 12 km and 21 km mountain trail run with water points every 4 km. Medals for all finishers.',
    startDate: Date.parse('2026-11-01T06:00:00Z'),
    endDate: Date.parse('2026-11-01T13:00:00Z'),
    distanceMin: 12,
    distanceMax: 21,
    address: {
      countryCode: 'PT',
      addressLine1: 'Parque de Monserrate',
      cityTownLocality: 'Sintra',
      administrativeArea: 'Lisboa',
      postalCode: '2710-405',
      latitude: 38.7939,
      longitude: -9.4209,
    },
    coverImage: image('trail-cover', 'Runners on a mountain path at sunrise'),
    images: [
      image('trail-1', 'Start line'),
      image('trail-2', 'Runner crossing a stream'),
      image('trail-3', 'Finish line medals'),
      image('trail-4', 'View from the summit'),
    ],
  },
  {
    id: 'h3k8w1z6r9m2v5t0b7n4c1x8',
    eventType: 'MOUNTAIN_BIKE',
    name: 'Arrábida Enduro Challenge',
    description: 'Five timed enduro stages along the Arrábida ridge, with shuttle uplift between stages and a riders\' braai at the finish.',
    startDate: Date.parse('2026-11-08T07:30:00Z'),
    endDate: Date.parse('2026-11-08T16:00:00Z'),
    distanceMin: 35,
    distanceMax: 35,
    address: {
      countryCode: 'PT',
      addressLine1: 'Estrada de Escarpa',
      cityTownLocality: 'Setúbal',
      sublocality: 'Azeitão',
      administrativeArea: 'Setúbal',
      postalCode: '2925-001',
    },
    coverImage: image('mtb-cover', 'Rider descending a rocky trail'),
    images: [image('mtb-1', 'Riders at the start ramp'), image('mtb-2', 'Bike wash station')],
  },
  {
    id: 'b9t4n7c2x5p8r1w6z3m0k9v2',
    eventType: 'EXHIBITION',
    name: 'Outdoor Gear Expo',
    description: 'Over 80 exhibitors showing the latest running, cycling and hiking gear, plus talks from pro athletes across both days.',
    startDate: Date.parse('2026-11-14T09:00:00Z'),
    endDate: Date.parse('2026-11-15T18:00:00Z'),
    address: {
      countryCode: 'DE',
      addressLine1: 'Messedamm 22',
      cityTownLocality: 'Berlin',
      sublocality: 'Westend',
      administrativeArea: 'Berlin',
      postalCode: '14055',
      latitude: 52.5065,
      longitude: 13.2846,
    },
    coverImage: image('expo-cover', 'Exhibition hall full of stands'),
    images: [
      image('expo-1', 'Shoe wall at a brand stand'),
      image('expo-2', 'Athlete talk on the main stage'),
      image('expo-3', 'Visitors trying out bikes'),
    ],
  },
  {
    id: 'm5r0w3z8k1v6t9b2n5c8x1p4',
    eventType: 'HIKE',
    name: 'Veluwe Autumn Hike',
    description: 'A guided hike through heathland and forest at peak autumn colour. Choose the short or long loop; both end at the visitor centre café.',
    startDate: Date.parse('2026-11-21T08:00:00Z'),
    endDate: Date.parse('2026-11-21T15:00:00Z'),
    distanceMin: 10,
    distanceMax: 25,
    address: {
      countryCode: 'NL',
      addressLine1: 'Houtkampweg 9',
      cityTownLocality: 'Otterlo',
      administrativeArea: 'Gelderland',
      postalCode: '6731 AV',
      latitude: 52.0977,
      longitude: 5.7737,
    },
    coverImage: image('hike-cover', 'Hikers crossing purple heathland'),
    images: [image('hike-1', 'Forest path in autumn'), image('hike-2', 'Group at the viewpoint')],
  },
  {
    id: 'c2x7p0r5w8z3m6k1v4t7b0n3',
    eventType: 'TRAIL_RUN',
    name: 'Harz Night Trail',
    description: 'A head-torch trail run up and over the Brocken. Mandatory kit: head torch, waterproof jacket and foil blanket.',
    startDate: Date.parse('2026-12-05T17:00:00Z'),
    endDate: Date.parse('2026-12-05T23:30:00Z'),
    distanceMin: 18,
    distanceMax: 42,
    address: {
      countryCode: 'DE',
      addressLine1: 'Brockenstraße 1',
      cityTownLocality: 'Schierke',
      administrativeArea: 'Sachsen-Anhalt',
      postalCode: '38879',
    },
    coverImage: image('night-cover', 'Head torches snaking up a dark mountain'),
    images: [image('night-1', 'Runners at the summit marker'), image('night-2', 'Aid station at night')],
  },
  {
    id: 'v8t1b4n9c6x3p0r7w2z5m8k1',
    eventType: 'MOUNTAIN_BIKE',
    name: 'Dunes & Dykes XC Marathon',
    description: 'A fast cross-country marathon through the coastal dunes, with 60 km and 100 km options.',
    startDate: Date.parse('2026-12-12T08:00:00Z'),
    distanceMin: 60,
    distanceMax: 100,
    address: {
      countryCode: 'NL',
      addressLine1: 'Zeeweg 1',
      addressLine2: 'Parkeerplaats Parnassia',
      cityTownLocality: 'Bloemendaal',
      administrativeArea: 'Noord-Holland',
      postalCode: '2051 EC',
    },
    coverImage: image('xc-cover', 'Mountain bikers riding through sand dunes'),
    images: [image('xc-1', 'Riders on a sandy singletrack'), image('xc-2', 'Podium ceremony')],
  },
  {
    id: 'k4v7t0b3n8c5x2p9r6w1z4m7',
    eventType: 'EXHIBITION',
    name: 'Mountain Photography Exhibition',
    description: 'Large-format prints from twenty alpine and trail photographers, with an opening night talk on shooting in the mountains.',
    startDate: Date.parse('2026-12-18T10:00:00Z'),
    endDate: Date.parse('2027-01-10T18:00:00Z'),
    address: {
      countryCode: 'PT',
      addressLine1: 'Rua da Boavista 132',
      cityTownLocality: 'Lisboa',
      sublocality: 'Misericórdia',
      administrativeArea: 'Lisboa',
      postalCode: '1200-066',
    },
    coverImage: image('photo-cover', 'Framed mountain photographs on a gallery wall'),
    images: [image('photo-1', 'Visitors at the opening night'), image('photo-2', 'Close-up of a glacier print')],
  },
]

// The backend lowercases and strips whitespace when building the `Ns` key.
const normalize = (s: string) => s.replace(/\s/g, '').toLowerCase()

const DEFAULT_LIMIT = 50

/**
 * Mimics the events service: matches on event type + country code only and
 * applies the limit. Like the backend, it ignores startDate/endDate for now and
 * does not sort or paginate.
 */
export function listMockEvents(events: SsEvent[], req: ListEventsRequest): SsEvent[] {
  const ns = normalize(`${req.eventType}~~${req.countryCode}`)
  const limit = req.limit && req.limit > 0 ? req.limit : DEFAULT_LIMIT
  return events
    .filter((event) => normalize(`${event.eventType}~~${event.address.countryCode}`) === ns)
    .slice(0, limit)
}
