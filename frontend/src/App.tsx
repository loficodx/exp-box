import { useState } from 'react'
import './App.css'
import { Rooms } from './Rooms'
import { RoomRce } from './RoomRce'

type View = 'rooms' | 'rce'

function App() {
  const [view, setView] = useState<View>('rooms')

  if (view === 'rce') {
    return <RoomRce onBack={() => setView('rooms')} />
  }

  return (
    <Rooms
      onSelectRoom={(slug) => {
        if (slug === 'rce') setView('rce')
      }}
    />
  )
}

export default App
