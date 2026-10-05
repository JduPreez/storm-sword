import type { SsEvent, EventImage } from '../../models'
import type { EventFilter } from './types'

// Stand-in for the backend until the events API exists. Delete this file once
// eventsApi.ts talks to the real API.

const image = (id: string, alt: string): EventImage => ({
  id,
  alt,
  // picsum.photos returns a stable random photo per seed.
  url: `https://picsum.photos/seed/${id}/1200/675`,
})

export const mockEvents: SsEvent[] = [
  {
    id: 'q7m2xk9v4t8b1n6c3p5r0w2z',
    title: 'Autumn Jazz Night',
    description:
      'An evening of live jazz with three local quartets, a late-night jam session and a small food market in the courtyard.',
    category: 'music',
    startsAt: '2026-10-17T19:30:00Z',
    venue: 'The Blue Room',
    city: 'Lisbon',
    coverImage: image('jazz-cover', 'Saxophone player on a dimly lit stage'),
    images: [
      image('jazz-1', 'Crowd in front of the stage'),
      image('jazz-2', 'Double bass close-up'),
      image('jazz-3', 'Courtyard food stalls'),
    ],
  },
  {
    id: 'h3k8w1z6r9m2v5t0b7n4c1x8',
    title: 'Rust Meetup: Async in Practice',
    description:
      'Two talks on async Rust in production services, followed by lightning talks and pizza. Bring your own war stories.',
    category: 'tech',
    startsAt: '2026-10-22T17:00:00Z',
    venue: 'Coworking Loft',
    city: 'Lisbon',
    coverImage: image('rust-cover', 'Speaker presenting code on a projector'),
    images: [image('rust-1', 'Audience during a talk'), image('rust-2', 'Laptop with code editor')],
  },
  {
    id: 'b9t4n7c2x5p8r1w6z3m0k9v2',
    title: 'Trail Run Series: Round 3',
    description: 'A 12 km and 21 km mountain trail run with water points every 4 km. Medals for all finishers.',
    category: 'sport',
    startsAt: '2026-11-01T06:00:00Z',
    venue: 'Ridge Trailhead',
    city: 'Lisbon',
    coverImage: image('trail-cover', 'Runners on a mountain path at sunrise'),
    images: [
      image('trail-1', 'Start line'),
      image('trail-2', 'Runner crossing a stream'),
      image('trail-3', 'Finish line medals'),
      image('trail-4', 'View from the summit'),
    ],
  },
  {
    id: 'm5r0w3z8k1v6t9b2n5c8x1p4',
    title: 'Street Food Festival',
    description: 'Forty food trucks, a craft beer garden and live DJs across two days.',
    category: 'food',
    startsAt: '2026-11-14T11:00:00Z',
    venue: 'Waterfront Plaza',
    city: 'Berlin',
    coverImage: image('food-cover', 'Food trucks lined up at dusk'),
    images: [image('food-1', 'Tacos on a tray'), image('food-2', 'Crowd at the beer garden')],
  },
  {
    id: 'c2x7p0r5w8z3m6k1v4t7b0n3',
    title: 'Open Studio Weekend',
    description: 'Thirty artists open their studios to the public. Maps are available at the main gallery.',
    category: 'art',
    startsAt: '2026-11-21T09:00:00Z',
    venue: 'Arts Quarter',
    city: 'Amsterdam',
    coverImage: image('studio-cover', 'Paintings hanging in a bright studio'),
    images: [
      image('studio-1', 'Sculptor at work'),
      image('studio-2', 'Visitors looking at prints'),
      image('studio-3', 'Ceramics on a shelf'),
    ],
  },
  {
    id: 'v8t1b4n9c6x3p0r7w2z5m8k1',
    title: 'Summer Sunset Concert',
    description: 'An open-air orchestral concert on the lawns. Picnic baskets welcome; no glass.',
    category: 'music',
    startsAt: '2026-12-05T18:00:00Z',
    venue: 'Botanical Gardens',
    city: 'Lisbon',
    coverImage: image('sunset-cover', 'Orchestra on an outdoor stage at sunset'),
    images: [image('sunset-1', 'Picnic blankets on the lawn'), image('sunset-2', 'Violin section')],
  },
  {
    id: 'k4v7t0b3n8c5x2p9r6w1z4m7',
    title: 'Frontend Friday: React 19 Deep Dive',
    description: 'Hands-on workshop covering Actions, use() and the new form hooks. Laptops required.',
    category: 'tech',
    startsAt: '2026-12-11T13:00:00Z',
    venue: 'Tech Hub',
    city: 'Amsterdam',
    coverImage: image('react-cover', 'Workshop attendees at laptops'),
    images: [image('react-1', 'Whiteboard diagram'), image('react-2', 'Pair programming')],
  },
]

/** Mimics the filtering the backend will eventually do. */
export function filterMockEvents(events: SsEvent[], filter: EventFilter): SsEvent[] {
  const search = filter.search?.trim().toLowerCase()
  return events
    .filter((event) => !filter.category || event.category === filter.category)
    .filter(
      (event) =>
        !search ||
        event.title.toLowerCase().includes(search) ||
        event.venue.toLowerCase().includes(search) ||
        event.city.toLowerCase().includes(search),
    )
    .sort((a, b) => a.startsAt.localeCompare(b.startsAt))
}
