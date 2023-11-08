import { allDocs, Doc } from 'contentlayer/generated'
import { sort } from 'fast-sort'

import Navbar from './_navbar'
import Sidebar from './_sidebar'

type SectionNameMap = {
  section_id: number
  section_name: string
}

const sectionNameMaps: SectionNameMap[] = [
  {
    section_id: 1,
    section_name: 'Getting started',
  },
  {
    section_id: 2,
    section_name: 'Installation',
  },
]

export default async function DocsLayout({ children }: React.PropsWithChildren) {
  // Group data by section_id
  const groupedData: { [key: string]: Doc[] } = allDocs.reduce((acc, currentValue) => {
    const sectionId = currentValue.section_id
    if (!acc[sectionId]) {
      acc[sectionId] = []
    }
    acc[sectionId].push(currentValue)
    return acc
  }, {})

  // Sort items within each group by order_id
  Object.keys(groupedData).forEach((sectionId) => {
    groupedData[sectionId] = groupedData[sectionId].sort((a, b) => a.order_id - b.order_id)
  })

  // Format and display the data
  const x = Object.keys(groupedData).forEach((sectionId) => {
    const sectionNameMapping = sectionNameMaps.find(
      (mapping) => mapping.section_id === parseInt(sectionId, 10)
    )
    const sectionName = sectionNameMapping
      ? sectionNameMapping.section_name
      : `Section ${sectionId}`
    console.log(sectionName)
    groupedData[sectionId].forEach((item) => {
      console.log(`  ${item.title} (order by ${item.order_id})`)
    })
  })

  const docs: Doc[] = sort(allDocs).asc((item) => item.section_id)
  const filtered = docs.map(({ title, url }) => ({ title, url }))

  return (
    <div className='flex min-h-screen flex-col bg-white dark:bg-black'>
      <Navbar />
      <div className='flex flex-col md:flex-row'>
        <Sidebar items={filtered} />
        {children}
      </div>
    </div>
  )
}
