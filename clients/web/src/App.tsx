import { Flex, Layout, Result, Tag, Typography } from 'antd'
import { Link, Navigate, Route, Routes } from 'react-router'
import { EventDetailPage, EventListPage } from './features/events'
import { useGetHealthQuery } from './services/healthApi'

const { Header, Content } = Layout
const { Title } = Typography

function ApiStatus() {
  const { data, isLoading, error } = useGetHealthQuery()

  if (isLoading) return <Tag>API: checking…</Tag>
  if (error) return <Tag color="error">API: unreachable</Tag>
  return <Tag color="success">API: {data?.status}</Tag>
}

function App() {
  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Header>
        <Flex align="center" justify="space-between" style={{ height: '100%' }}>
          <Link to="/events">
            <Title level={3} style={{ color: 'white', margin: 0 }}>
              Storm Sword
            </Title>
          </Link>
          <ApiStatus />
        </Flex>
      </Header>
      <Content style={{ padding: 24 }}>
        <div style={{ maxWidth: 960, margin: '0 auto' }}>
          <Routes>
            <Route path="/" element={<Navigate to="/events" replace />} />
            <Route path="/events" element={<EventListPage />} />
            <Route path="/events/:eventId" element={<EventDetailPage />} />
            <Route path="*" element={<Result status="404" title="Page not found" />} />
          </Routes>
        </div>
      </Content>
    </Layout>
  )
}

export default App
