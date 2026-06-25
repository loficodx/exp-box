import { useState, useEffect } from 'react'
import { roomSubmitUrl } from './api'
import type { User } from './api'

interface Post {
  id: string
  title: string
  body: string
}

interface Comment {
  id: number
  user_id: string
  username: string
  body: string
  created_at: string
}

interface Props {
  user: User | null
  onBack: () => void
  onLogin: () => void
}

export function RoomXss({ user, onBack, onLogin }: Props) {
  const [post, setPost] = useState<Post | null>(null)
  const [comments, setComments] = useState<Comment[]>([])
  const [newComment, setNewComment] = useState('')
  const [commentError, setCommentError] = useState<string | null>(null)
  const [commentSubmitting, setCommentSubmitting] = useState(false)

  const [newPassword, setNewPassword] = useState('')
  const [passwordMsg, setPasswordMsg] = useState<string | null>(null)
  const [passwordSubmitting, setPasswordSubmitting] = useState(false)

  const [resetting, setResetting] = useState(false)
  const [resetMsg, setResetMsg] = useState<string | null>(null)

  const [flag, setFlag] = useState('')
  const [submitResult, setSubmitResult] = useState<boolean | null>(null)
  const [submitting, setSubmitting] = useState(false)

  useEffect(() => {
    fetchPost()
    fetchComments()
  }, [])

  async function fetchPost() {
    try {
      const res = await fetch('/api/rooms/xss/post')
      if (res.ok) setPost(await res.json())
    } catch {}
  }

  async function fetchComments() {
    try {
      const res = await fetch('/api/rooms/xss/comments', { credentials: 'same-origin' })
      if (res.ok) setComments(await res.json())
    } catch {}
  }

  async function submitComment() {
    if (!newComment.trim()) return
    setCommentSubmitting(true)
    setCommentError(null)
    try {
      const res = await fetch('/api/rooms/xss/comments', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'same-origin',
        body: JSON.stringify({ body: newComment }),
      })
      if (res.ok) {
        setNewComment('')
        await fetchComments()
      } else {
        const data = await res.json().catch(() => ({}))
        setCommentError(data.message ?? data.error ?? 'Failed to post comment')
      }
    } catch {
      setCommentError('Could not reach server')
    } finally {
      setCommentSubmitting(false)
    }
  }

  async function changePassword() {
    if (!newPassword.trim()) return
    setPasswordSubmitting(true)
    setPasswordMsg(null)
    try {
      const res = await fetch('/api/rooms/xss/change-password', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'same-origin',
        body: JSON.stringify({ password: newPassword }),
      })
      const data = await res.json().catch(() => ({}))
      if (res.ok) {
        setPasswordMsg('Password updated (fake room account only).')
        setNewPassword('')
      } else {
        setPasswordMsg(data.message ?? data.error ?? 'Failed to change password')
      }
    } catch {
      setPasswordMsg('Could not reach server')
    } finally {
      setPasswordSubmitting(false)
    }
  }

  async function resetRoom() {
    setResetting(true)
    setResetMsg(null)
    try {
      const res = await fetch('/api/rooms/xss/reset', {
        method: 'POST',
        credentials: 'same-origin',
      })
      if (res.ok) {
        setResetMsg('Room state reset.')
        await fetchComments()
      } else {
        const data = await res.json().catch(() => ({}))
        setResetMsg(data.message ?? data.error ?? 'Reset failed')
      }
    } catch {
      setResetMsg('Could not reach server')
    } finally {
      setResetting(false)
    }
  }

  async function submitFlag() {
    if (!flag.trim()) return
    setSubmitting(true)
    setSubmitResult(null)
    try {
      const res = await fetch(roomSubmitUrl('xss'), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'same-origin',
        body: JSON.stringify({ flag }),
      })
      const data = await res.json().catch(() => ({}))
      if (!res.ok) {
        setSubmitResult(false)
        return
      }
      setSubmitResult(data.correct)
    } catch {
      setSubmitResult(false)
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <div className="page">
      <button className="back-btn" onClick={onBack}>← Rooms</button>
      <h1>XSS / CSRF Training Room</h1>
      <p className="label">
        This room demonstrates stored XSS and CSRF vulnerabilities in a controlled
        environment. Objectives: (1) post a comment containing an XSS payload that
        executes JavaScript; (2) use CSRF to change the fake room password via the
        vulnerable form.
      </p>

      {post && (
        <section className="room-section">
          <h2>{post.title}</h2>
          <p>{post.body}</p>
        </section>
      )}

      <section className="room-section">
        <h2>Comments</h2>
        <div className="xss-comments">
          {comments.length === 0 && (
            <p className="status">No comments yet. Be the first!</p>
          )}
          {comments.map((c) => (
            <div key={c.id} className="xss-comment">
              <span className="xss-comment-author">{c.username}</span>
              {/* Intentionally vulnerable for the XSS/CSRF training room. */}
              {/* Do not reuse this pattern outside this controlled lab page. */}
              <span
                className="xss-comment-body"
                dangerouslySetInnerHTML={{ __html: c.body }}
              />
            </div>
          ))}
        </div>
        {user ? (
          <div style={{ marginTop: 16 }}>
            <div className="input-row">
              <input
                className="cmd-input"
                type="text"
                placeholder='Try: <img src=x onerror="alert(1)">'
                value={newComment}
                onChange={(e) => setNewComment(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && submitComment()}
              />
              <button
                className="action-btn"
                onClick={submitComment}
                disabled={commentSubmitting}
              >
                {commentSubmitting ? 'Posting…' : 'Post'}
              </button>
            </div>
            {commentError && <p className="auth-error" style={{ marginTop: 8 }}>{commentError}</p>}
          </div>
        ) : (
          <p className="auth-switch" style={{ marginTop: 12 }}>
            <button className="link-btn" onClick={onLogin}>Log in</button> to post a comment.
          </p>
        )}
      </section>

      <section className="room-section">
        <h2>Change Room Password</h2>
        <p className="xss-info">
          This form is intentionally missing CSRF protection — a state-changing
          endpoint that accepts requests from any origin. Craft a cross-site request
          that changes the fake room password without the victim clicking Submit.
        </p>
        {user ? (
          <>
            <div className="input-row">
              <input
                className="cmd-input"
                type="text"
                placeholder="New room password"
                value={newPassword}
                onChange={(e) => setNewPassword(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && changePassword()}
              />
              <button
                className="action-btn"
                onClick={changePassword}
                disabled={passwordSubmitting}
              >
                {passwordSubmitting ? 'Saving…' : 'Change'}
              </button>
            </div>
            {passwordMsg && <p className="status" style={{ marginTop: 8 }}>{passwordMsg}</p>}
          </>
        ) : (
          <p className="auth-switch">
            <button className="link-btn" onClick={onLogin}>Log in</button> to use this form.
          </p>
        )}
      </section>

      <section className="room-section">
        <h2>Submit Flag</h2>
        {user ? (
          <>
            <div className="input-row">
              <input
                className="cmd-input"
                type="text"
                placeholder="EXPBOX{...}"
                value={flag}
                onChange={(e) => setFlag(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && submitFlag()}
              />
              <button
                className="action-btn"
                onClick={submitFlag}
                disabled={submitting}
              >
                {submitting ? 'Checking…' : 'Submit'}
              </button>
            </div>
            {submitResult === true && (
              <p className="submit-result correct">Correct! Room solved.</p>
            )}
            {submitResult === false && (
              <p className="submit-result wrong">Incorrect flag — try again.</p>
            )}
          </>
        ) : (
          <p className="auth-switch">
            <button className="link-btn" onClick={onLogin}>Log in</button> to submit a flag.
          </p>
        )}
      </section>

      {user && (
        <section className="room-section">
          <h2>Reset Room</h2>
          <p className="xss-info">
            Clears your comments and resets your fake room password. Does not affect
            other users or your platform account.
          </p>
          <button
            className="action-btn"
            onClick={resetRoom}
            disabled={resetting}
            style={{ background: 'transparent', border: '1px solid currentColor', color: 'inherit' }}
          >
            {resetting ? 'Resetting…' : 'Reset my room state'}
          </button>
          {resetMsg && <p className="status" style={{ marginTop: 8 }}>{resetMsg}</p>}
        </section>
      )}
    </div>
  )
}