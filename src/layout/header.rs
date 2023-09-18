use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    let links: Vec<(&str, &str)> = vec![
      ("Services", "/services"),
      ("Work", "/work"),
      ("Why Aviabird", "/why-aviabird"),
      ("Blogs", "/blogs"),
      ("Opensource", "/opensource"),
      ("Contact Us", "/contact/hire-us"),
    ];

    view! {cx,
      <nav class="flex items-center justify-between flex-wrap container mx-auto p-6">
        <div class="flex items-center flex-shrink-0 text-gray-700 mr-6">
          <img class="h-[30px] mr-2" src="/assets/logo.jpg" />
          <span class="font-semibold text-2xl tracking-tight">Aviabird</span>
        </div>
        <div class="block lg:hidden">
          <button class="flex items-center px-3 py-2 border rounded text-teal-200 border-teal-400 hover:text-indigo-600 hover:border-teal-600">
            <i class="fa-solid fa-bars"></i>
          </button>
        </div>
        <div class="w-full block flex-grow lg:flex lg:items-center lg:text-right lg:w-auto">
          <div class="text-sm lg:flex-grow">
            {
              links.into_iter()
                .map(|(name, url)| view! {cx, 
                  <a href=url class="block mt-4 lg:inline-block lg:mt-0 text-gray-500 hover:text-gray-700 mr-8">
                    {name}
                  </a>
                })
                .collect_view(cx)
            }
          </div>
        </div>
      </nav>
    }
}
