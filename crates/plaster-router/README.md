# Usage
```rust
use plaster_router::{Routes, route_to};

#[derive(Routes)]
pub enum MyRoutes {
  #[route("/posts")]
  Posts,
  #[route("/posts/:id")]
  Post { id: String },
}

pub struct MyComponent {
  router: Router<MyRoutes>,
}

impl Component for MyComponent {
  fn create(_:_, mut context: ComponentLink<Self>) -> MyComponent {
    let mut router = MyRoutes::router(context.send_back(|| Msg::RouteUpdate));

    MyComponent {
      router: router,
    }
  }

  fn update(msg: Msg) -> ShouldRender {
    match msg {
      Msg::RouteUpdate => true,
      Msg::RouteTo(route) => { route_to(&route); true },
    }
  }
}

impl Renderable<MyComponent> on MyComponent {
  fn view(&self) -> Html<MyComponent> {
    match self.router.resolve() {
      Some(MyRoutes::Posts) => html! {
        <button onclick=|_| Msg::RouteTo("/posts/1".to_string()),>Post 1</button>
      },
      Some(MyRoutes::Post { id }) => html! {
        <h1>{format!("Post {}", id)}</h1>
      },
      None => html! {
        <h1>404 Not Found</h1>
      }
    }
  }
}
```
