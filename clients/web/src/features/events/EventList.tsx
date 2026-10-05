import { Empty, Listy } from 'antd'
import type { SsEvent } from '../../models'
import { formatEventMonth } from './display'
import EventListItem from './EventListItem'

interface EventListProps {
  events: SsEvent[]
}

function EventList({ events }: EventListProps) {
  if (events.length === 0) {
    return <Empty description="No events match your filters" />
  }

  return (
    <Listy
      items={events}
      rowKey="id"
      // Group by month with sticky headers. Events arrive sorted by date, so
      // the groups come out in chronological order.
      group={{
        key: (event) => formatEventMonth(event.startsAt),
        title: (month) => month,
      }}
      sticky
      itemRender={(event) => <EventListItem event={event} />}
    />
  )
}

export default EventList
