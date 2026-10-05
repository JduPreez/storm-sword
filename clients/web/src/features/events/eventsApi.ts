import type { SsEvent } from '../../models'
import { api } from '../../services/api'
import { filterMockEvents, mockEvents } from './mockEvents'
import type { EventFilter } from './types'

// Simulated network latency so loading states are visible while mocked.
const MOCK_DELAY_MS = 400
const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

export const eventsApi = api.injectEndpoints({
  endpoints: (builder) => ({
    // `queryFn` returns mock data instead of calling the API. To hook up the
    // backend, replace it with something like:
    //   query: (filter) => ({ url: '/events', params: filter }),
    getEvents: builder.query<SsEvent[], EventFilter>({
      queryFn: async (filter) => {
        await delay(MOCK_DELAY_MS)
        return { data: filterMockEvents(mockEvents, filter) }
      },
    }),
    // Real version: query: (id) => `/events/${id}`
    getEvent: builder.query<SsEvent, string>({
      queryFn: async (id) => {
        await delay(MOCK_DELAY_MS)
        const event = mockEvents.find((e) => e.id === id)
        return event ? { data: event } : { error: { status: 404, data: 'Event not found' } }
      },
    }),
  }),
})

export const { useGetEventsQuery, useGetEventQuery } = eventsApi
