import { useState, type FormEvent } from 'react'
import { login, type User, getMe } from './api'

interface Props {
  onSuccess: (user: User) => void
  onRegister: () => void
}

export function Login({ onSuccess, onRegister }: Props) {
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [error, setError] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)

  async function handleSubmit(e: FormEvent) {
    e.preventDefault()
    if (!username.trim() || !password) return
    setLoading(true)
    setError(null)
    const result = await login(username.trim(), password)
    if (!result.ok) {
      setError(result.error ?? 'Login failed')
      setLoading(false)
      return
    }
    const user = await getMe()
    setLoading(false)
    if (user) onSuccess(user)
    else setError('Login succeeded but session could not be read')
  }

  return (
    <div className="auth-page">
      <h1>Login</h1>
      <form className="auth-form" onSubmit={handleSubmit}>
        <label className="auth-label">
          Username
          <input
            className="auth-input"
            type="text"
            autoComplete="username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            disabled={loading}
          />
        </label>
        <label className="auth-label">
          Password
          <input
            className="auth-input"
            type="password"
            autoComplete="current-password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            disabled={loading}
          />
        </label>
        {error && <p className="auth-error">{error}</p>}
        <button className="action-btn" type="submit" disabled={loading}>
          {loading ? 'Logging in…' : 'Login'}
        </button>
      </form>
      <p className="auth-switch">
        No account?{' '}
        <button className="link-btn" onClick={onRegister}>Register</button>
      </p>
    </div>
  )
}