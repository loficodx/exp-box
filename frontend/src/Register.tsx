import { useState, type FormEvent } from 'react'
import { register, login, type User, getMe } from './api'

interface Props {
  onSuccess: (user: User) => void
  onLogin: () => void
}

export function Register({ onSuccess, onLogin }: Props) {
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [error, setError] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)

  async function handleSubmit(e: FormEvent) {
    e.preventDefault()
    if (!username.trim() || !password) return
    setLoading(true)
    setError(null)
    const result = await register(username.trim(), password)
    if (!result.ok) {
      setError(result.error ?? 'Registration failed')
      setLoading(false)
      return
    }
    const loginResult = await login(username.trim(), password)
    if (!loginResult.ok) {
      setError('Registered but auto-login failed — please log in manually')
      setLoading(false)
      return
    }
    const user = await getMe()
    setLoading(false)
    if (user) onSuccess(user)
    else setError('Registered but session could not be read')
  }

  return (
    <div className="auth-page">
      <h1>Register</h1>
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
            autoComplete="new-password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            disabled={loading}
          />
        </label>
        {error && <p className="auth-error">{error}</p>}
        <button className="action-btn" type="submit" disabled={loading}>
          {loading ? 'Registering…' : 'Register'}
        </button>
      </form>
      <p className="auth-switch">
        Already have an account?{' '}
        <button className="link-btn" onClick={onLogin}>Login</button>
      </p>
    </div>
  )
}