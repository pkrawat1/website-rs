use leptos::*;

#[component]
pub fn Header(cx: Scope, dark_mode: RwSignal<bool>) -> impl IntoView {
    let mobile_menu = create_rw_signal(cx, false);
    let links: Vec<(&str, &str)> = vec![
        ("Services", "/services"),
        ("Work", "/work"),
        ("Why Aviabird", "/why-aviabird"),
        ("Blogs", "/blogs"),
        ("Opensource", "/opensource"),
    ];

    view! {cx,
          <div class="w-full">
            <nav class="container relative flex flex-wrap items-center justify-between p-8 mx-auto lg:justify-between xl:px-0">
              <div class="flex flex-wrap items-center justify-between w-full lg:w-auto">
                <a href="/">
                  <span class="flex items-center space-x-2 text-2xl font-medium text-indigo-500 dark:text-gray-100">
                    <span>
                      <img alt="N" loading="lazy" width="32" height="32" decoding="async" data-nimg="1" class="w-8"
                        style="color:transparent" src="/assets/logo.png" />
                    </span>
                    <span>
                      Aviabird
                    </span>
                  </span>
                </a>
                <button on:click=move |_| mobile_menu.update(|v| *v=!*v) aria-label="Toggle Menu"
                  class="px-2 py-1 ml-auto text-gray-500 rounded-md lg:hidden hover:text-indigo-500 focus:text-indigo-500 focus:bg-indigo-100 focus:outline-none dark:text-gray-300 dark:focus:bg-trueGray-700"
                  id="headlessui-disclosure-button-:R956:" type="button" aria-expanded="false" data-headlessui-state="">
                  <i class="fa-solid fa-bars fa-xl dark:text-white" class=("fa-xmark", mobile_menu) />
                </button>
                <div class="flex flex-wrap w-full my-5 lg:hidden" class:hidden=move || !mobile_menu()>
                  {
                    links.clone().into_iter()
                      .map(|(name, url)| view! {cx,
                        <a class="w-full px-4 py-2 -ml-4 text-gray-500 rounded-md dark:text-gray-300 hover:text-indigo-500 focus:text-indigo-500 focus:bg-indigo-100 dark:focus:bg-gray-800 focus:outline-none"
                          href=url
                        >
                          {name}
                        </a>
                      })
                      .collect_view(cx)
                  }
                  <a class="px-6 py-2 mt-2 text-white text-center bg-indigo-600 rounded-md md:ml-5 w-full"
                    href="/contact/hire-us">
                    Get Started
                  </a>
                </div>
              </div>
              <div class="hidden text-center lg:flex lg:items-center">
                <ul class="items-center justify-end flex-1 pt-6 list-none lg:pt-0 lg:flex">
                  {
                    links.clone().into_iter()
                      .map(|(name, url)| view! {cx,
                        <li class="mr-3 nav__item">
                          <a class="inline-block px-4 py-2 text-lg font-normal text-gray-800 no-underline rounded-md dark:text-gray-200 hover:text-indigo-500 focus:text-indigo-500 focus:bg-indigo-100 focus:outline-none dark:focus:bg-gray-800"
                            href=url
                          >
                            {name}
                          </a>
                        </li>
                      })
                      .collect_view(cx)
                  }
                </ul>
              </div>
              <div class="hidden mr-3 space-x-4 lg:flex nav__item">
                <a class="px-6 py-2 text-white bg-indigo-600 rounded-md md:ml-5" href="/contact/hire-us">
                  Get Started
                </a>
                <div class="flex items-center">
                  <button on:click=move |_| dark_mode.update(|value| *value=!*value)>
                    <i class="fa-solid fa-sun fa-xl w-8" class:fa-moon=move || !dark_mode() class:text-white=move || dark_mode()
                      class:text-white=move || dark_mode() />
                  </button>
                </div>
              </div>
            </nav>
          </div>
    }
}
