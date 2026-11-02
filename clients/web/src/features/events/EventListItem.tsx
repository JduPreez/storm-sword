import { Flex, Tag, Typography, theme } from 'antd'
import { Link } from 'react-router'
import type { SsEvent } from '../../models'
import { EVENT_TYPE_COLORS, EVENT_TYPE_LABELS, formatEventDate } from './display'

const { Text } = Typography

interface EventListItemProps {
  event: SsEvent
}

const THUMBNAIL_STYLE = { width: 120, height: 68, borderRadius: 6, flexShrink: 0 }

function EventListItem({ event }: EventListItemProps) {
  const { token } = theme.useToken()

  return (
    <Link to={`/events/${event.id}`} style={{ display: 'block', color: 'inherit' }}>
      <Flex gap="middle" align="center">
        {event.coverImage?.url ? (
          <img
            src={event.coverImage.url}
            alt={event.coverImage.alt}
            loading="lazy"
            style={{ ...THUMBNAIL_STYLE, objectFit: 'cover' }}
          />
        ) : (
          // Placeholder of the same size so rows without a cover stay aligned.
          <div style={{ ...THUMBNAIL_STYLE, background: token.colorFillSecondary }} />
        )}
        <Flex vertical gap={4} style={{ minWidth: 0 }}>
          <Text strong ellipsis>
            {event.name}
          </Text>
          <Text type="secondary" ellipsis>
            {formatEventDate(event.startDate)} ·{' '}
            {[event.address?.cityTownLocality, event.address?.administrativeArea].filter(Boolean).join(', ')}
          </Text>
          <div>
            <Tag color={EVENT_TYPE_COLORS[event.eventType]}>{EVENT_TYPE_LABELS[event.eventType]}</Tag>
          </div>
        </Flex>
      </Flex>
    </Link>
  )
}

export default EventListItem
