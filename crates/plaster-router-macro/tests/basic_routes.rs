use plaster_router_macro::Routes;

#[test]
fn test_basic_route() {
    #[derive(Routes)]
    enum BasicRoute {
        #[route("/route1")]
        Route1,
    }
}

#[test]
fn test_route_with_param() {
    #[derive(Routes)]
    enum BasicRoute {
        #[route("/route1/:param")]
        Route1 { param: String },
    }
}
