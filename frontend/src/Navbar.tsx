import { type User, logout } from './api'

interface Props {
  user: User | null
  onLogin: () => void
  onRegister: () => void
  onLogout: () => void
}

export function Navbar({ user, onLogin, onRegister, onLogout }: Props) {
  async function handleLogout() {
    await logout()
    onLogout()
  }

  return (
    <nav className="navbar">
      <span className="navbar-brand">exp-box</span>
      <div className="navbar-actions">
        {user ? (
          <>
            <span className="navbar-user">{user.username}</span>
            <button className="nav-btn" onClick={handleLogout}>Logout</button>
          </>
        ) : (
          <>
            <button className="nav-btn" onClick={onLogin}>Login</button>
            <button className="nav-btn nav-btn-primary" onClick={onRegister}>Register</button>
          </>
        )}
      </div>
    </nav>
  )
}