import { useState, useEffect } from 'react'

interface Room {
  slug: string
  title: string
  category: string
  difficulty: string
  position: number
  description: string
  solved: boolean
}

interface Props {
  onSelectRoom: (slug: string) => void
}

export function Rooms({ onSelectRoom }: Props) {
  const [rooms, setRooms] = useState<Room[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    fetch('/api/rooms')
      .then((res) => {
        if (!res.ok) throw new Error('Failed to load rooms')
        return res.json()
      })
      .then((data: Room[]) => {
        setRooms(data)
        setLoading(false)
      })
      .catch(() => {
        setError('Could not load rooms')
        setLoading(false)
      })
  }, [])

  return (
    <div className="page">
      <h1>exp-box</h1>
      <p className="label">Training rooms</p>
      {loading && <p className="status">Loading…</p>}
      {error && <p className="status error">{error}</p>}
      <div className="room-grid">
        {rooms.map((room) => (
          <button
            key={room.slug}
            className="room-card"
            onClick={() => onSelectRoom(room.slug)}
          >
            <div className="room-card-header">
              <span className="room-title">{room.title}</span>
              {room.solved && <span className="badge solved">Solved</span>}
            </div>
            <div className="room-meta">
              <span className="badge category">{room.category}</span>
              <span className="badge difficulty">{room.difficulty}</span>
            </div>
            <p className="room-desc">{room.description}</p>
          </button>
        ))}
      </div>
    </div>
  )
}
