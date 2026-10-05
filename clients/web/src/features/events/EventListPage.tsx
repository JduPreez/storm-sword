import { Alert, Flex, Spin, Typography } from 'antd'
import { useSearchParams } from 'react-router'
import { EVENT_CATEGORIES, type EventCategory } from '../../models'
import EventFilters from './EventFilters'
import EventList from './EventList'
import { useGetEventsQuery } from './eventsApi'
import type { EventFilter } from './types'

const { Title } = Typography

const isCategory = (value: string | null): value is EventCategory =>
  EVENT_CATEGORIES.includes(value as EventCategory)

// The filter lives in the URL (?search=...&category=...), so it survives
// reloads, can be shared as a link, and is restored when coming back from
// the detail view.
function readFilter(params: URLSearchParams): EventFilter {
  const category = params.get('category')
  return {
    search: params.get('search') ?? undefined,
    category: isCategory(category) ? category : undefined,
  }
}

function writeFilter(filter: EventFilter): URLSearchParams {
  const params = new URLSearchParams()
  if (filter.search) params.set('search', filter.search)
  if (filter.category) params.set('category', filter.category)
  return params
}

// Container: owns the filter state and data fetching, passes plain props down.
function EventListPage() {
  const [searchParams, setSearchParams] = useSearchParams()
  const filter = readFilter(searchParams)
  const { data: events = [], isFetching, error } = useGetEventsQuery(filter)

  return (
    <Flex vertical gap="middle">
      <Title level={2} style={{ margin: 0 }}>
        Events
      </Title>
      <EventFilters value={filter} onChange={(next) => setSearchParams(writeFilter(next), { replace: true })} />
      {error && <Alert type="error" title="Failed to load events" showIcon />}
      <Spin spinning={isFetching}>
        <EventList events={events} />
      </Spin>
    </Flex>
  )
}

export default EventListPage
