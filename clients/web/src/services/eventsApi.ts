import { api } from './api';
import type { SsEvent, DateTime } from '../models/index';
import * as R from 'ramda';

export interface ListEventsRequest {
  eventType: string;
  countryCode: string;
  startDate?: string;
  endDate?: string;
  limit?: number;
}

export interface ListEventsResponse {
  events: SsEvent[];
  nextToken: string;
}

const toIso =
  R.cond<[DateTime | undefined], string | null>([
    [R.isNil, R.always(null)],
    [R.is(Number), (ms) => new Date(ms as number).toISOString()],
    [R.T, (d) => `${d}`],
  ]);

// Dates arrive as epoch numbers and the address has no display string, so
// normalise each event into the shape the UI expects.
export const toSsEvent = (e: SsEvent): SsEvent => {
  const description = [
    ...(e.address?.addressLine1 ? [e.address.addressLine1]: []),
    ...(e.address?.addressLine2 ? [e.address.addressLine2]: []),
    ...(e.address?.cityTownLocality ? [e.address.cityTownLocality]: []),
    ...(e.address?.administrativeArea ? [e.address.administrativeArea]: []),
    ...(e.address?.postalCode ? [e.address.postalCode]: [])
  ].join(', ');

  return {
    ...e,
    startDate: toIso(e.startDate),
    endDate: toIso(e.endDate),
    address: e.address && { ...e.address, description },
  };
};

export const transformListEventsResponse = (response: ListEventsResponse): ListEventsResponse => ({
  ...response,
  events: response.events.map(toSsEvent),
});

export const eventsApi = api.injectEndpoints({
  endpoints: (builder) => ({
    listEvents: builder.query<ListEventsResponse, ListEventsRequest>({
      query: ({ eventType, countryCode, ...params }) => ({
        url: `/events/${encodeURIComponent(eventType)}/${encodeURIComponent(countryCode)}`,
        params, // RTK's fetchBaseQuery drops undefined values
      }),
      transformResponse: transformListEventsResponse,
    }),
  }),
});

export const { useListEventsQuery } = eventsApi;