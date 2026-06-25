INSERT INTO rooms (slug, title, category, difficulty, position, description, flag_hash)
VALUES (
  'xss',
  'Stored XSS & CSRF',
  'web',
  'medium',
  2,
  'A blog comment section stores and reflects user input unsanitised. A password-change form accepts cross-origin requests without CSRF protection. Chain the two vulnerabilities to claim the flag.',
  '75bad52017e5ff9357382531d8e9aabec3da48c3944cbf5e9d8bfb84a15238a1'
)
ON CONFLICT (slug) DO NOTHING;
