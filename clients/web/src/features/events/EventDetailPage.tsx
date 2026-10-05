import { Button, Col, Descriptions, Flex, Image, Result, Row, Spin, Tag, Typography, theme } from 'antd'
import { Link, useParams } from 'react-router'
import { CATEGORY_COLORS, CATEGORY_LABELS, formatEventDate } from './display'
import { useGetEventQuery } from './eventsApi'

const { Title, Paragraph } = Typography

function EventDetailPage() {
  const { eventId = '' } = useParams()
  const { data: event, isLoading, error } = useGetEventQuery(eventId)
  // react-router's <Link> is a plain <a>, which antd doesn't style, so take the
  // link colour from the theme.
  const { token } = theme.useToken()

  if (isLoading) {
    return <Spin />
  }

  if (error || !event) {
    return (
      <Result
        status="404"
        title="Event not found"
        extra={
          <Link to="/events">
            <Button type="primary">Back to events</Button>
          </Link>
        }
      />
    )
  }

  return (
    <Flex vertical gap="middle">
      <Link to="/events" style={{ color: token.colorLink }}>
        ← All events
      </Link>

      <Flex align="center" gap="small" wrap>
        <Title level={2} style={{ margin: 0 }}>
          {event.title}
        </Title>
        <Tag color={CATEGORY_COLORS[event.category]}>{CATEGORY_LABELS[event.category]}</Tag>
      </Flex>

      <Image
        src={event.coverImage.url}
        alt={event.coverImage.alt}
        width="100%"
        style={{ aspectRatio: '16 / 9', objectFit: 'cover', borderRadius: 8 }}
      />

      <Descriptions
        column={{ xs: 1, sm: 3 }}
        items={[
          { key: 'when', label: 'When', children: formatEventDate(event.startsAt) },
          { key: 'venue', label: 'Venue', children: event.venue },
          { key: 'city', label: 'City', children: event.city },
        ]}
      />

      <Paragraph>{event.description}</Paragraph>

      {event.images.length > 0 && (
        <>
          <Title level={4} style={{ margin: 0 }}>
            Gallery
          </Title>
          {/* PreviewGroup lets the user page through all gallery images in the lightbox. */}
          <Image.PreviewGroup>
            <Row gutter={[12, 12]}>
              {event.images.map((image) => (
                <Col key={image.id} xs={12} sm={8}>
                  <Image
                    src={image.url}
                    alt={image.alt}
                    width="100%"
                    style={{ aspectRatio: '16 / 9', objectFit: 'cover', borderRadius: 6 }}
                  />
                </Col>
              ))}
            </Row>
          </Image.PreviewGroup>
        </>
      )}
    </Flex>
  )
}

export default EventDetailPage
