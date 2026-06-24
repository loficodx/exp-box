export function roomActionUrl(slug: string, action: string) {
  return `/api/rooms/${slug}/actions/${action}`
}

export function roomSubmitUrl(slug: string) {
  return `/api/rooms/${slug}/submit`
}
