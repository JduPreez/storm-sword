import { Alert, Layout, Spin, Typography } from 'antd'
import { useGetHealthQuery } from './services/healthApi'

const { Header, Content } = Layout
const { Title, Text } = Typography

function App() {
  const { data, isLoading, error } = useGetHealthQuery()

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Header>
        <Title level={3} style={{ color: 'white', margin: 0, lineHeight: '64px' }}>
          storm-sword
        </Title>
      </Header>
      <Content style={{ padding: 24 }}>
        {isLoading && <Spin />}
        {error && <Alert type="error" message="Failed to reach the API" showIcon />}
        {data && (
          <Text>
            API status: <Text strong>{data.status}</Text>
          </Text>
        )}
      </Content>
    </Layout>
  )
}

export default App
