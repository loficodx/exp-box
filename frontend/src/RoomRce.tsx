import { useState } from 'react'

interface Props {
  onBack: () => void
}

export function RoomRce({ onBack }: Props) {
  const [cmd, setCmd] = useState('')
  const [output, setOutput] = useState<string | null>(null)
  const [running, setRunning] = useState(false)

  const [flag, setFlag] = useState('')
  const [submitResult, setSubmitResult] = useState<boolean | null>(null)
  const [submitting, setSubmitting] = useState(false)

  async function runCmd() {
    if (!cmd.trim()) return
    setRunning(true)
    setOutput(null)
    try {
      const res = await fetch('/api/rooms/rce/exec', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ cmd }),
      })
      const data: { stdout: string; stderr: string } = await res.json()
      setOutput((data.stdout || '') + (data.stderr ? '\n[stderr]\n' + data.stderr : ''))
    } catch {
      setOutput('Error: could not reach backend')
    } finally {
      setRunning(false)
    }
  }

  async function submitFlag() {
    if (!flag.trim()) return
    setSubmitting(true)
    setSubmitResult(null)
    try {
      const res = await fetch('/api/rooms/rce/submit', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ flag }),
      })
      const data: { correct: boolean } = await res.json()
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
      <h1>Remote Code Execution</h1>
      <p className="label">
        This server exposes a diagnostics utility that executes any command you
        send. Your objective: find the flag hidden on the server and submit it below.
      </p>

      <section className="room-section">
        <h2>Console</h2>
        <div className="input-row">
          <input
            className="cmd-input"
            type="text"
            placeholder="e.g. id"
            value={cmd}
            onChange={(e) => setCmd(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && runCmd()}
          />
          <button className="action-btn" onClick={runCmd} disabled={running}>
            {running ? 'Running…' : 'Run'}
          </button>
        </div>
        {output !== null && (
          <pre className="terminal">{output || '(no output)'}</pre>
        )}
      </section>

      <section className="room-section">
        <h2>Submit flag</h2>
        <div className="input-row">
          <input
            className="cmd-input"
            type="text"
            placeholder="EXPBOX{...}"
            value={flag}
            onChange={(e) => setFlag(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && submitFlag()}
          />
          <button className="action-btn" onClick={submitFlag} disabled={submitting}>
            {submitting ? 'Checking…' : 'Submit'}
          </button>
        </div>
        {submitResult === true && (
          <p className="submit-result correct">Correct! Room solved.</p>
        )}
        {submitResult === false && (
          <p className="submit-result wrong">Incorrect flag — try again.</p>
        )}
      </section>
    </div>
  )
}
