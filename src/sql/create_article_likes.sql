CREATE TABLE IF NOT EXISTS article_likes (
  name TEXT NOT NULL,
  ip TEXT NOT NULL,
  date INTEGER NOT NULL,
  UNIQUE(name, ip)
)
