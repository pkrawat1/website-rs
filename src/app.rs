use crate::layout::footer::*;
use crate::layout::header::*;
use crate::pages::home::home::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let dark_mode = create_rw_signal(cx, true);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/website_rs.css"/>
        <link defer rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.2/css/all.min.css" />

        // sets the document title
        <Title text="Aviabird Technologies Pvt. Ltd."/>

        // content for this welcome page
        <Router>
            <main class:dark=dark_mode>
              <div class="dark:bg-neutral-900">
                <Header dark_mode=dark_mode/>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=HomePage/>
                </Routes>
                <Footer />
              </div>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}
