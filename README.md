# Workers benchmark

Deploy minimum worker code for JavaScript and Rust(wasm) and compare them.

Original code are generated by

- `wrangler generate`
- `wrangler generate type=rust`

and align implementation details.

## Code
### javascript
```javascript
addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Respond with hello worker text
 * @param {Request} request
 */
async function handleRequest(request) {
  return new Response(`Hello worker! at ${request.url}`, {
    headers: { 'content-type': 'text/plain' },
  })
}
```

### rust
```rust
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: worker::Context) -> Result<Response> {
    let res = String::from("Hello worker! at ") + &req.url()?.to_string();
    Response::ok(res)
}
```

## Size
### javascript
```
└── [ 313]  index.js
```

### rust
```
├── [  71]  index.js
└── [    ]  worker
    ├── [ 15K]  index_bg.mjs
    ├── [340K]  index_bg.wasm
    └── [ 182]  shim.mjs
```

## Performance

By `autocannon -c 5 -d 5 URL`.

### javascript

```
┌─────────┬──────┬───────┬───────┬───────┬──────────┬─────────┬────────┐
│ Stat    │ 2.5% │ 50%   │ 97.5% │ 99%   │ Avg      │ Stdev   │ Max    │
├─────────┼──────┼───────┼───────┼───────┼──────────┼─────────┼────────┤
│ Latency │ 9 ms │ 11 ms │ 21 ms │ 25 ms │ 12.31 ms │ 7.34 ms │ 262 ms │
└─────────┴──────┴───────┴───────┴───────┴──────────┴─────────┴────────┘
┌───────────┬────────┬────────┬────────┬────────┬────────┬─────────┬────────┐
│ Stat      │ 1%     │ 2.5%   │ 50%    │ 97.5%  │ Avg    │ Stdev   │ Min    │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Req/Sec   │ 372    │ 372    │ 393    │ 408    │ 390    │ 14.33   │ 372    │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Bytes/Sec │ 273 kB │ 273 kB │ 289 kB │ 300 kB │ 286 kB │ 10.5 kB │ 273 kB │
└───────────┴────────┴────────┴────────┴────────┴────────┴─────────┴────────┘

Req/Bytes counts sampled once per second.
# of samples: 5

2k requests in 5.05s, 1.43 MB read
```

### rust
```
┌─────────┬──────┬───────┬───────┬───────┬──────────┬─────────┬────────┐
│ Stat    │ 2.5% │ 50%   │ 97.5% │ 99%   │ Avg      │ Stdev   │ Max    │
├─────────┼──────┼───────┼───────┼───────┼──────────┼─────────┼────────┤
│ Latency │ 9 ms │ 12 ms │ 24 ms │ 29 ms │ 13.02 ms │ 9.67 ms │ 374 ms │
└─────────┴──────┴───────┴───────┴───────┴──────────┴─────────┴────────┘
┌───────────┬────────┬────────┬────────┬────────┬────────┬─────────┬────────┐
│ Stat      │ 1%     │ 2.5%   │ 50%    │ 97.5%  │ Avg    │ Stdev   │ Min    │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Req/Sec   │ 323    │ 323    │ 379    │ 387    │ 369.2  │ 23.35   │ 323    │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Bytes/Sec │ 233 kB │ 233 kB │ 273 kB │ 279 kB │ 266 kB │ 16.8 kB │ 232 kB │
└───────────┴────────┴────────┴────────┴────────┴────────┴─────────┴────────┘

Req/Bytes counts sampled once per second.
# of samples: 5

2k requests in 5.04s, 1.33 MB read
```
