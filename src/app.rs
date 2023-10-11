use crate::layout::{footer::*, header::*};
use crate::pages::{blogs::*, contact::*, home::*};
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
        <link defer rel="stylesheet"
          href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.2/css/all.min.css"
          media="none"
          onload="if(media!=='all')media='all'"  />

        // sets the document title
        <Title text="Aviabird Technologies Pvt. Ltd."/>

        <div class:dark=dark_mode>
          <Router>
              <main class="dark:bg-neutral-900">
                <Header dark_mode=dark_mode/>
                <section class="min-h-[50vh]">
                  <Routes>
                      <Route path="" view=HomePage/>
                      <Route path="/contact/hire-us" view=HireUsPage/>
                      <Route path="/blogs" view=BlogsPage/>
                      <Route path="/*any" view=ComingSoon />
                  </Routes>
                </section>
                <Footer />
              </main>
          </Router>
        </div>
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

#[component]
fn ComingSoon(cx: Scope) -> impl IntoView {
    view! {cx,
      <div class="flex flex-col h-screen dark:bg-neutral-900 dark:text-white">
        <div class="grid place-items-center w-4/5 mx-auto p-10 my-20 sm:my-auto bg-white-600 border-4 border-indigo-600 bg-opacity-70 rounded-xl shadow-2xl space-y-5 text-center cursor-pointer">
            <i class="fa text-9xl fa-person-digging text-orange-600 dark:fill-white" />
            <h1 class="text-4xl font-bold uppercase text-orange-600 transition duration-500">"Coming Soon"</h1>
            <h2 class="text-xl text-gray-700 dark:text-white transition duration-500">
                "We are almost there to introduce our new website to the world."
            </h2>
            <div class="flex justify-center">
                <a href="/contact/hire-us" title="Contact Us"
                    class="tracking-wide font-bold rounded border-2 border-indigo-500 hover:text-white hover:border-blue-600 hover:bg-blue-600 shadow-md py-2 px-6 inline-flex items-center transition duration-500">
                    <span class="mx-auto">"Contact Us"</span>
                </a>
            </div>
        </div>
    </div>
    }
}
