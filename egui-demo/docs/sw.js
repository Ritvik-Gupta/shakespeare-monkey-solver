var cacheName = "egui-template-pwa"
var filesToCache = [
	"./",
	"./index.html",
	"./egui_demo.js",
	"./egui_demo_bg.wasm",
]

/* Start the service worker and cache all of the app's content */
self.addEventListener("install", event =>
	event.waitUntil(caches.open(cacheName).then(cache => cache.addAll(filesToCache)))
)

/* Serve cached content when offline */
self.addEventListener("fetch", event =>
	event.respondWith(caches.match(event.request).then(response => response ?? fetch(event.request)))
)
