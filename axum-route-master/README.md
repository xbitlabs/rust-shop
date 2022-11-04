# axum-route

```rust
use axum::routing::get;
use axum_route::route;

#[route(get, "/")]
pub async fn index() -> String {
    "Hello World!".into_string()
}
```
