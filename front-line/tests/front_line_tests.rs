use front_line::{Error, FrontLine, HttpVersion, Router, RouterResult};
use rstest::rstest;

#[derive(PartialEq, Debug, FrontLine)]
enum MarketingRoutes {
    #[get("/")]
    RenderIndex,
    #[get("/sign-up")]
    RenderSignUp,
    #[post("/sign-up")]
    ProcessSignUp,
    #[get("/log-in")]
    RenderLogIn,
    #[post("/log-in")]
    ProcessLogIn,
    #[get("/portal")]
    RenderPortal,
}

#[derive(PartialEq, Debug, FrontLine)]
#[prefix("/api")]
enum ApiRoutes<'a> {
    #[get("/users")]
    GetAllUsers,
    #[post("/users")]
    CreateUser,
    #[get("/users/{id}")]
    GetUser { id: u32 },
    #[get("/users/{id}/roles/{role}")]
    GetUserRole { id: u32, role: &'a str },
    #[put("/users/{id}/roles/{role}")]
    UpdateUserRole { id: u32, role: &'a str },
}

#[derive(PartialEq, Debug, FrontLine)]
enum AllRoutes<'a> {
    #[flatten]
    Marketing(MarketingRoutes),
    #[flatten]
    Api(ApiRoutes<'a>),
}

#[rstest]
#[case(
    b"GET / HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(MarketingRoutes::RenderIndex),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"GET /?key=value HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(MarketingRoutes::RenderIndex),
        query: "key=value",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"GET / HTTP/1.1\r\n\r\nheader-section",
        Ok(RouterResult {
        route: Some(MarketingRoutes::RenderIndex),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"header-section",
    })
)]
#[case(
    b"GET /?key=value HTTP/1.1\r\n\r\nheader-section",
    Ok(RouterResult {
        route: Some(MarketingRoutes::RenderIndex),
        query: "key=value",
        version: HttpVersion::OneOne,
        head_and_body: b"header-section",
    })
)]
#[case(
    b"GET /sign-up HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(MarketingRoutes::RenderSignUp),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"POST /sign-up HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(MarketingRoutes::ProcessSignUp),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"GET /log-in HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(MarketingRoutes::RenderLogIn),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"POST /log-in HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(MarketingRoutes::ProcessLogIn),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"GET /portal HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(MarketingRoutes::RenderPortal),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
fn test_non_prefixed_routes(
    #[case] input: &[u8],
    #[case] expected_result: Result<RouterResult<'_, MarketingRoutes>, Error>,
) {
    let result = MarketingRoutes::resolve(input);
    assert_eq!(result, expected_result);
}

#[rstest]
#[case(
    b"GET /api/users HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(ApiRoutes::GetAllUsers),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"POST /api/users HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(ApiRoutes::CreateUser),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"GET /api/users/42 HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(ApiRoutes::GetUser { id: 42 }),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"GET /api/users/42/roles/admin HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(ApiRoutes::GetUserRole { id: 42, role: "admin" }),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"PUT /api/users/42/roles/admin HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(ApiRoutes::UpdateUserRole { id: 42, role: "admin" }),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"PUT /api/users/42/roles/admin?key=value HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(ApiRoutes::UpdateUserRole { id: 42, role: "admin" }),
        query: "key=value",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"PUT /api/users/42/roles/admin HTTP/1.1\r\n\r\nheader-section",
    Ok(RouterResult {
        route: Some(ApiRoutes::UpdateUserRole { id: 42, role: "admin" }),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"header-section",
    })
)]
#[case(
    b"PUT /api/users/42/roles/admin?key=value HTTP/1.1\r\n\r\nheader-section",
    Ok(RouterResult {
        route: Some(ApiRoutes::UpdateUserRole { id: 42, role: "admin" }),
        query: "key=value",
        version: HttpVersion::OneOne,
        head_and_body: b"header-section",
    })
)]
fn test_prefixed_routes(
    #[case] input: &[u8],
    #[case] expected_result: Result<RouterResult<'_, ApiRoutes>, Error>,
) {
    let result = ApiRoutes::resolve(input);
    assert_eq!(result, expected_result);
}

#[rstest]
#[case(
    b"GET / HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(AllRoutes::Marketing(MarketingRoutes::RenderIndex)),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"GET /?key=value HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(AllRoutes::Marketing(MarketingRoutes::RenderIndex)),
        query: "key=value",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"GET / HTTP/1.1\r\n\r\nheader-section",
        Ok(RouterResult {
        route: Some(AllRoutes::Marketing(MarketingRoutes::RenderIndex)),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"header-section",
    })
)]
#[case(
    b"GET /?key=value HTTP/1.1\r\n\r\nheader-section",
    Ok(RouterResult {
        route: Some(AllRoutes::Marketing(MarketingRoutes::RenderIndex)),
        query: "key=value",
        version: HttpVersion::OneOne,
        head_and_body: b"header-section",
    })
)]
#[case(
    b"GET /sign-up HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(AllRoutes::Marketing(MarketingRoutes::RenderSignUp)),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"POST /sign-up HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(AllRoutes::Marketing(MarketingRoutes::ProcessSignUp)),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"GET /log-in HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(AllRoutes::Marketing(MarketingRoutes::RenderLogIn)),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"POST /log-in HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(AllRoutes::Marketing(MarketingRoutes::ProcessLogIn)),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"GET /portal HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(AllRoutes::Marketing(MarketingRoutes::RenderPortal)),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"GET /api/users HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(AllRoutes::Api(ApiRoutes::GetAllUsers)),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"POST /api/users HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(AllRoutes::Api(ApiRoutes::CreateUser)),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"GET /api/users/42 HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(AllRoutes::Api(ApiRoutes::GetUser { id: 42 })),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"GET /api/users/42/roles/admin HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(AllRoutes::Api(ApiRoutes::GetUserRole { id: 42, role: "admin" })),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"PUT /api/users/42/roles/admin HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(AllRoutes::Api(ApiRoutes::UpdateUserRole { id: 42, role: "admin" })),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"PUT /api/users/42/roles/admin?key=value HTTP/1.1\r\n\r\n",
    Ok(RouterResult {
        route: Some(AllRoutes::Api(ApiRoutes::UpdateUserRole { id: 42, role: "admin" })),
        query: "key=value",
        version: HttpVersion::OneOne,
        head_and_body: b"",
    })
)]
#[case(
    b"PUT /api/users/42/roles/admin HTTP/1.1\r\n\r\nheader-section",
    Ok(RouterResult {
        route: Some(AllRoutes::Api(ApiRoutes::UpdateUserRole { id: 42, role: "admin" })),
        query: "",
        version: HttpVersion::OneOne,
        head_and_body: b"header-section",
    })
)]
#[case(
    b"PUT /api/users/42/roles/admin?key=value HTTP/1.1\r\n\r\nheader-section",
    Ok(RouterResult {
        route: Some(AllRoutes::Api(ApiRoutes::UpdateUserRole { id: 42, role: "admin" })),
        query: "key=value",
        version: HttpVersion::OneOne,
        head_and_body: b"header-section",
    })
)]
fn test_merged_routes(
    #[case] input: &[u8],
    #[case] expected_result: Result<RouterResult<'_, AllRoutes>, Error>,
) {
    let result = AllRoutes::resolve(input);
    assert_eq!(result, expected_result);
}
