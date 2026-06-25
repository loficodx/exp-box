export function roomActionUrl(slug: string, action: string) {
  return `/api/rooms/${slug}/actions/${action}`
}

export function roomSubmitUrl(slug: string) {
  return `/api/rooms/${slug}/submit`
}

export interface User {
  user_id: string
  username: string
}

export async function getMe(): Promise<User | null> {
  try {
    const res = await fetch('/api/auth/me', { credentials: 'same-origin' })
    if (!res.ok) return null
    return res.json()
  } catch {
    return null
  }
}

export async function login(username: string, password: string): Promise<{ ok: boolean; error?: string }> {
  try {
    const res = await fetch('/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'same-origin',
      body: JSON.stringify({ username, password }),
    })
    if (res.ok) return { ok: true }
    const data = await res.json().catch(() => ({}))
    return { ok: false, error: data.message ?? data.error ?? 'Login failed' }
  } catch {
    return { ok: false, error: 'Could not reach server' }
  }
}

export async function register(username: string, password: string): Promise<{ ok: boolean; error?: string }> {
  try {
    const res = await fetch('/api/auth/register', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'same-origin',
      body: JSON.stringify({ username, password }),
    })
    if (res.ok) return { ok: true }
    const data = await res.json().catch(() => ({}))
    return { ok: false, error: data.message ?? data.error ?? 'Registration failed' }
  } catch {
    return { ok: false, error: 'Could not reach server' }
  }
}

export async function logout(): Promise<void> {
  await fetch('/api/auth/logout', {
    method: 'POST',
    credentials: 'same-origin',
  })
}
