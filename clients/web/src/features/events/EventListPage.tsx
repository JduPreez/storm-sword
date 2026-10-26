import { skipToken } from '@reduxjs/toolkit/query/react'
import { Alert, Empty, Flex, Spin, Typography } from 'antd'
import { useMemo } from 'react'
import { useSearchParams } from 'react-router'
import { EVENT_TYPES, type DateTime, type EventType, type SsEvent } from '../../models'
import { COUNTRY_CODES } from './display'
import EventFilters from './EventFilters'
import EventList from './EventList'
import { useListEventsQuery } from './eventsApi' // '../../services/eventsApi'
import type { EventFilter } from './types'

const { Title } = Typography

const isEventType = (value: string | null): value is EventType =>
  EVENT_TYPES.includes(value as EventType)

const isCountryCode = (value: string | null): value is string =>
  COUNTRY_CODES.includes(value as (typeof COUNTRY_CODES)[number])

const toTime = (date: DateTime | undefined) => (date == null ? Infinity : new Date(date).getTime())

// The API has no search and doesn't sort, so both happen here on the page of
// results it returns. EventList relies on the date order to group by month.
function searchAndSort(events: SsEvent[], search: string | undefined): SsEvent[] {
  const term = search?.trim().toLowerCase()
  return events
    .filter(
      (event) =>
        !term ||
        [event.name, event.address?.cityTownLocality, event.address?.administrativeArea].some((value) =>
          value?.toLowerCase().includes(term),
        ),
    )
    .sort((a, b) => toTime(a.startDate) - toTime(b.startDate))
}

// The filter lives in the URL (?search=...&eventType=...&countryCode=...), so it survives
// reloads, can be shared as a link, and is restored when coming back from
// the detail view.
function readFilter(params: URLSearchParams): EventFilter {
  const eventType = params.get('eventType')
  const countryCode = params.get('countryCode')
  return {
    search: params.get('search') ?? undefined,
    eventType: isEventType(eventType) ? eventType : undefined,
    countryCode: isCountryCode(countryCode) ? countryCode : undefined,
  }
}

function writeFilter(filter: EventFilter): URLSearchParams {
  const params = new URLSearchParams()
  if (filter.search) params.set('search', filter.search)
  if (filter.eventType) params.set('eventType', filter.eventType)
  if (filter.countryCode) params.set('countryCode', filter.countryCode)
  return params
}

// Container: owns the filter state and data fetching, passes plain props down.
function EventListPage() {
  const [searchParams, setSearchParams] = useSearchParams()
  const filter = readFilter(searchParams)
  const { search, eventType, countryCode } = filter
  // Both are path params on the API, so there is nothing to fetch until the
  // user has picked them.
  const canQuery = Boolean(eventType && countryCode)
  const { data, isFetching, error } = useListEventsQuery(
    eventType && countryCode ? { eventType, countryCode } : skipToken,
  )
  const events = useMemo(() => searchAndSort(data?.events ?? [], search), [data, search])

  return (
    <Flex vertical gap="middle">
      <Title level={2} style={{ margin: 0 }}>
        Events
      </Title>
      <EventFilters value={filter} onChange={(next) => setSearchParams(writeFilter(next), { replace: true })} />
      {error && <Alert type="error" title="Failed to load events" showIcon />}
      {canQuery ? (
        <Spin spinning={isFetching}>
          <EventList events={events} />
        </Spin>
      ) : (
        <Empty description="Choose an event type and country to see events" />
      )}
    </Flex>
  )
}

export default EventListPage
