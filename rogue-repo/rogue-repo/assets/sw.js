/** Copyright (c) 2026 The Cochran Block. All rights reserved. */
const CACHE = 'rogue-repo-v4';
self.addEventListener('install', (e) => {
  e.waitUntil(caches.open(CACHE).then((c) => c.addAll([
    '/',
    '/manifest.json',
    '/apps/rogue-runner',
    '/apps/rogue-runner-wasm',
    '/login',
    '/register',
    '/assets/apps/rogue-runner.png',
    '/assets/apps/rogue-runner-wasm/rogue-runner.wasm'
  ])));
});
self.addEventListener('fetch', (e) => {
  e.respondWith(caches.match(e.request).then((r) => r || fetch(e.request)));
});
