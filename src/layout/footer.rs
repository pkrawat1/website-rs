use leptos::*;
#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    let open_source_links = vec![
        ("Angular Spree", "https://github.com/aviabird/angularspree"),
        ("Aviacommerce", "https://github.com/aviacommerce/avia"),
        ("Gringotts", "https://github.com/aviabird/gringotts"),
        ("Yatrum", "https://github.com/aviabird/yatrum"),
        ("PinWork", "https://github.com/aviabird/pinterest"),
        ("Angular Seed", "https://github.com/aviabird/angular-seed"),
    ];
    let technologies = vec!["Elixir", "Rust", "Go", "DevOps", "JavaScript"];
    let industries = vec!["E-commerce", "Fintech", "Insurtech", "Weather Intelligence", "Warehousing"];

    view! {cx,
      <div class="relative">
        <div class="container p-8 mx-auto xl:px-0">
          <div
            class="grid max-w-screen-xl grid-cols-1 gap-10 pt-10 mx-auto mt-5 border-t border-gray-100 dark:border-trueGray-700 lg:grid-cols-5"
          >
            <div class="lg:col-span-2">
              <div>
                <a
                  class="flex items-center space-x-2 text-2xl font-medium text-indigo-500 dark:text-gray-100"
                  href="/"
                  ><img
                    alt="N"
                    loading="lazy"
                    width="32"
                    height="32"
                    decoding="async"
                    data-nimg="1"
                    class="w-8"
                    style="color: transparent"
                    src="/assets/logo.png"
                  />
                  <span>Aviabird</span>
                </a>
              </div>
              <div class="max-w-md mt-4 text-gray-500 dark:text-gray-400">
                Aviabird is a leading IT consultancy with specialising in various
                technologies. While delivering products we have built a solid track
                record of our capabilities.
              </div>
            </div>
            <div>
              <div class="flex flex-wrap w-full -mt-2 -ml-3 lg:ml-0">
                <legend class="text-xl w-full px-4 py-2 text-gray-500 rounded-md dark:text-gray-300 hover:text-indigo-500 focus:text-indigo-500 focus:bg-indigo-100 focus:outline-none dark:focus:bg-trueGray-700">Open Source</legend>
                {
                  open_source_links.into_iter().map(|(name, url)|
                    view! {cx,
                      <a
                        class="w-full px-4 py-2 text-gray-500 rounded-md dark:text-gray-300 hover:text-indigo-500 focus:text-indigo-500 focus:bg-indigo-100 focus:outline-none dark:focus:bg-trueGray-700"
                        href=url
                        >{name}</a>
                    }
                  ).collect_view(cx)
                }
              </div>
            </div>
            <div>
              <div class="flex flex-wrap w-full -mt-2 -ml-3 lg:ml-0">
                <legend class="text-xl w-full px-4 py-2 text-gray-500 rounded-md dark:text-gray-300 hover:text-indigo-500 focus:text-indigo-500 focus:bg-indigo-100 focus:outline-none dark:focus:bg-trueGray-700">
                  Technologies
                </legend>
                {
                  technologies.into_iter().map(|name|
                    view! {cx,
                      <a
                        class="w-full px-4 py-2 text-gray-500 rounded-md dark:text-gray-300 hover:text-indigo-500 focus:text-indigo-500 focus:bg-indigo-100 focus:outline-none dark:focus:bg-trueGray-700"
                        href="#"
                        >{name}</a>
                    }
                  ).collect_view(cx)
                }
              </div>
            </div>
            <div>
              <div class="flex flex-wrap w-full -mt-2 -ml-3 lg:ml-0">
                <legend class="text-xl w-full px-4 py-2 text-gray-500 rounded-md dark:text-gray-300 hover:text-indigo-500 focus:text-indigo-500 focus:bg-indigo-100 focus:outline-none dark:focus:bg-trueGray-700">
                  Industries
                </legend>
                {
                  industries.into_iter().map(|name|
                    view! {cx,
                      <a
                        class="w-full px-4 py-2 text-gray-500 rounded-md dark:text-gray-300 hover:text-indigo-500 focus:text-indigo-500 focus:bg-indigo-100 focus:outline-none dark:focus:bg-trueGray-700"
                        href="#"
                        >{name}</a>
                    }
                  ).collect_view(cx)
                }
              </div>
            </div>
          </div>
          <div class="my-10 text-sm text-center text-gray-600 dark:text-gray-400">
            Copyright <i class="fa-solid fa-copyright" /> 2023. Aviabird Technologies
            Pvt. Ltd. All Rights Reserved
          </div>
        </div>
      </div>
    }
}
