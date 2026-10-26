import type { SsEvent } from '../../models'
import { api } from '../../services/api'
import {
  toSsEvent,
  transformListEventsResponse,
  type ListEventsRequest,
  type ListEventsResponse,
} from '../../services/eventsApi'
import { listMockEvents, mockEvents } from './mockEvents'

// Mock of services/eventsApi.ts. `useListEventsQuery` has the same request and
// response types and runs the same transform, so switching between the two is
// just a change of import path.

// Simulated network latency so loading states are visible while mocked.
const MOCK_DELAY_MS = 400
const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

export const eventsApi = api.injectEndpoints({
  endpoints: (builder) => ({
    // Named differently from the real `listEvents` so both can be injected
    // into the same api without clashing.
    listMockEvents: builder.query<ListEventsResponse, ListEventsRequest>({
      queryFn: async (req) => {
        await delay(MOCK_DELAY_MS)
        return { data: transformListEventsResponse({ events: listMockEvents(mockEvents, req), nextToken: '' }) }
      },
    }),
    // The backend has no get-by-id endpoint yet.
    getEvent: builder.query<SsEvent, string>({
      queryFn: async (id) => {
        await delay(MOCK_DELAY_MS)
        const event = mockEvents.find((e) => e.id === id)
        return event ? { data: toSsEvent(event) } : { error: { status: 404, data: 'Event not found' } }
      },
    }),
  }),
})

export const { useListMockEventsQuery: useListEventsQuery, useGetEventQuery } = eventsApi
