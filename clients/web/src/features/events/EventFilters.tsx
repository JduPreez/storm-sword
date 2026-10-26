import { Flex, Input, Select } from 'antd'
import { EVENT_TYPES } from '../../models'
import { COUNTRY_CODES, EVENT_TYPE_LABELS, formatCountry } from './display'
import type { EventFilter } from './types'

interface EventFiltersProps {
  value: EventFilter
  onChange: (value: EventFilter) => void
}

const eventTypeOptions = EVENT_TYPES.map((eventType) => ({
  value: eventType,
  label: EVENT_TYPE_LABELS[eventType],
}))

const countryOptions = COUNTRY_CODES.map((countryCode) => ({
  value: countryCode,
  label: formatCountry(countryCode),
}))

// Presentational: knows nothing about the API or the URL, it just reports
// filter changes to its parent.
function EventFilters({ value, onChange }: EventFiltersProps) {
  return (
    <Flex gap="small" wrap>
      <Input.Search
        placeholder="Search name, city or region"
        allowClear
        // Uncontrolled so typing doesn't trigger a query per keystroke;
        // the filter only changes on Enter, the search button or clear.
        defaultValue={value.search}
        onSearch={(search) => onChange({ ...value, search: search || undefined })}
        style={{ maxWidth: 320 }}
      />
      <Select
        placeholder="Event type"
        allowClear
        value={value.eventType}
        onChange={(eventType) => onChange({ ...value, eventType })}
        options={eventTypeOptions}
        style={{ width: 200 }}
      />
      <Select
        placeholder="Country"
        allowClear
        showSearch={{ optionFilterProp: 'label' }}
        value={value.countryCode}
        onChange={(countryCode) => onChange({ ...value, countryCode })}
        options={countryOptions}
        style={{ width: 200 }}
      />
    </Flex>
  )
}

export default EventFilters
