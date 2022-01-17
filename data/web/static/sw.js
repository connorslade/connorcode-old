const staticCacheName = "ConnorCode";
const cachedItems = [
  "/assets/fonts/Inter-Regular.woff",
  "/assets/fonts/BasementGrotesque-Black_v1.202.woff",
  "/assets/fonts/Aileron-SemiBold.woff",
  "/assets/fonts/JetBrainsMono-Regular.woff2",

  "/assets/pageImg/0b0t-Map.png",
  "/assets/pageImg/desktop.png",
  "/assets/pageImg/ideagen.png",
  "/assets/pageImg/colorgen.png",
  "/assets/pageImg/EAmap.png",
  "/assets/pageImg/Github.png",
  "/assets/pageImg/christmas.png",
  "/assets/pageImg/FOTD_Bot.png",
  "/assets/pageImg/lakeWaterTemp.png",
  "/assets/pageImg/contact.png",
  "/assets/pageImg/checklist.png",
  "/assets/pageImg/2b2tQ.png",

  "/favicon.ico",
  "/manifest.json",
  "/sw-loader.js"
];

self.addEventListener("install", (e) => {
  e.waitUntil(
    caches.open(staticCacheName).then((cache) => {
      return cache.addAll(cachedItems);
    })
  );
});

self.addEventListener("fetch", (event) => {
  event.respondWith(
    caches.match(event.request).then((response) => {
      return response || fetch(event.request);
    })
  );
});
