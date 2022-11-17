const staticCacheName = "ConnorCode";
const cachedItems = [
  // Fonts
  "/assets/fonts/Inter-Regular.woff",
  "/assets/fonts/BasementGrotesque-Black_v1.202.woff",
  "/assets/fonts/Aileron-SemiBold.woff",
  "/assets/fonts/JetBrainsMono-Regular.woff2",

  // Webp Images
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

  // Png Versions
  "/assets/pageImg/0b0t-Map.png.webp",
  "/assets/pageImg/desktop.png.webp",
  "/assets/pageImg/ideagen.png.webp",
  "/assets/pageImg/colorgen.png.webp",
  "/assets/pageImg/EAmap.png.webp",
  "/assets/pageImg/Github.png.webp",
  "/assets/pageImg/christmas.png.webp",
  "/assets/pageImg/FOTD_Bot.png.webp",
  "/assets/pageImg/lakeWaterTemp.png.webp",
  "/assets/pageImg/contact.png.webp",
  "/assets/pageImg/checklist.png.webp",
  "/assets/pageImg/2b2tQ.png.webp",

  // Misc
  "/favicon.ico",
  "/manifest.json",
  "/sw-loader.js",
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
