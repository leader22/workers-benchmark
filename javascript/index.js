addEventListener("fetch", (ev) => {
  ev.respondWith(handleEvent(ev))
})

async function handleEvent() {
  return new Response("Hello worker!");
}
