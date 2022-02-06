CREATE TABLE IF NOT EXISTS article_views (
  name TEXT NOT NULL,
  ip TEXT NOT NULL,
  date INTEGER NOT NULL,
  UNIQUE(name, ip)
)
