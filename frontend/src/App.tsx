import { useState, useEffect } from 'react'
import './App.css'

const API_BASE_URL = (import.meta.env.VITE_API_BASE_URL || '').replace(/\/$/, '')

function App() {
  const [message, setMessage] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    fetch(`${API_BASE_URL}/api/health`)
      .then((res) => {
        if (!res.ok) throw new Error('Backend returned an error')
        return res.json()
      })
      .then((data: { message: string }) => {
        setMessage(data.message)
        setLoading(false)
      })
      .catch(() => {
        setError('Backend is not available')
        setLoading(false)
      })
  }, [])

  return (
    <div className="container">
      <h1>exp-box</h1>
      <p className="label">Backend status:</p>
      {loading && <p className="status">Loading…</p>}
      {error && <p className="status error">{error}</p>}
      {message && <p className="status">{message}</p>}
    </div>
  )
}

export default App
