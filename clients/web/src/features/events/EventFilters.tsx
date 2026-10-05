import { Flex, Input, Select } from 'antd'
import { EVENT_CATEGORIES } from '../../models'
import { CATEGORY_LABELS } from './display'
import type { EventFilter } from './types'

interface EventFiltersProps {
  value: EventFilter
  onChange: (value: EventFilter) => void
}

const categoryOptions = EVENT_CATEGORIES.map((category) => ({
  value: category,
  label: CATEGORY_LABELS[category],
}))

// Presentational: knows nothing about the API or the URL, it just reports
// filter changes to its parent.
function EventFilters({ value, onChange }: EventFiltersProps) {
  return (
    <Flex gap="small" wrap>
      <Input.Search
        placeholder="Search title, venue or city"
        allowClear
        // Uncontrolled so typing doesn't trigger a query per keystroke;
        // the filter only changes on Enter, the search button or clear.
        defaultValue={value.search}
        onSearch={(search) => onChange({ ...value, search: search || undefined })}
        style={{ maxWidth: 320 }}
      />
      <Select
        placeholder="All categories"
        allowClear
        value={value.category}
        onChange={(category) => onChange({ ...value, category })}
        options={categoryOptions}
        style={{ width: 200 }}
      />
    </Flex>
  )
}

export default EventFilters
