import { Flex, Tag, Typography } from 'antd'
import { Link } from 'react-router'
import type { SsEvent } from '../../models'
import { CATEGORY_COLORS, CATEGORY_LABELS, formatEventDate } from './display'

const { Text } = Typography

interface EventListItemProps {
  event: SsEvent
}

function EventListItem({ event }: EventListItemProps) {
  return (
    <Link to={`/events/${event.id}`} style={{ display: 'block', color: 'inherit' }}>
      <Flex gap="middle" align="center">
        <img
          src={event.coverImage.url}
          alt={event.coverImage.alt}
          width={120}
          height={68}
          loading="lazy"
          style={{ objectFit: 'cover', borderRadius: 6, flexShrink: 0 }}
        />
        <Flex vertical gap={4} style={{ minWidth: 0 }}>
          <Text strong ellipsis>
            {event.title}
          </Text>
          <Text type="secondary" ellipsis>
            {formatEventDate(event.startsAt)} · {event.venue}, {event.city}
          </Text>
          <div>
            <Tag color={CATEGORY_COLORS[event.category]}>{CATEGORY_LABELS[event.category]}</Tag>
          </div>
        </Flex>
      </Flex>
    </Link>
  )
}

export default EventListItem
