import { useState, useEffect } from 'react'
import './App.css'
import { type User, getMe } from './api'
import { Navbar } from './Navbar'
import { Login } from './Login'
import { Register } from './Register'
import { Rooms } from './Rooms'
import { RoomRce } from './RoomRce'
import { RoomXss } from './RoomXss'

type View = 'rooms' | 'rce' | 'xss' | 'login' | 'register'

function App() {
  const [view, setView] = useState<View>('rooms')
  const [user, setUser] = useState<User | null>(null)
  const [authChecked, setAuthChecked] = useState(false)

  useEffect(() => {
    getMe().then((u) => {
      setUser(u)
      setAuthChecked(true)
    })
  }, [])

  function handleLogout() {
    setUser(null)
    setView('rooms')
  }

  function handleAuthSuccess(u: User) {
    setUser(u)
    setView('rooms')
  }

  if (!authChecked) return null

  let content: React.ReactNode

  if (view === 'login') {
    content = (
      <Login
        onSuccess={handleAuthSuccess}
        onRegister={() => setView('register')}
      />
    )
  } else if (view === 'register') {
    content = (
      <Register
        onSuccess={handleAuthSuccess}
        onLogin={() => setView('login')}
      />
    )
  } else if (view === 'rce') {
    content = <RoomRce onBack={() => setView('rooms')} />
  } else if (view === 'xss') {
    content = (
      <RoomXss
        user={user}
        onBack={() => setView('rooms')}
        onLogin={() => setView('login')}
      />
    )
  } else {
    content = (
      <Rooms
        onSelectRoom={(slug) => {
          if (slug === 'rce') setView('rce')
          else if (slug === 'xss') setView('xss')
        }}
      />
    )
  }

  return (
    <>
      <Navbar
        user={user}
        onLogin={() => setView('login')}
        onRegister={() => setView('register')}
        onLogout={handleLogout}
      />
      {content}
    </>
  )
}

export default App