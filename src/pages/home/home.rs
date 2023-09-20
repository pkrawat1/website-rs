use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
      <Banner />
    }
}

#[component]
fn Banner(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="container p-8 mx-auto xl:px-0 flex flex-wrap ">
         <div class="flex items-center w-full lg:w-1/2">
            <div class="max-w-2xl mb-8">
               <h1 class="text-4xl font-bold leading-snug tracking-tight text-gray-800 lg:text-4xl lg:leading-tight xl:text-6xl xl:leading-tight dark:text-white">"Crafting Tomorrow's Software Solutions Today"</h1>
               <p class="py-5 text-xl leading-normal text-gray-500 lg:text-xl xl:text-2xl dark:text-gray-300">"At Aviabird, Our expert team is dedicated to delivering cutting-edge software solutions that drive innovation and transform businesses. Let's build the future together."</p>
               <div class="flex flex-col items-start space-y-3 sm:space-x-4 sm:space-y-0 sm:items-center sm:flex-row">
                  <a href="/contact/hire-us" class="px-8 py-4 text-lg font-medium text-center text-white bg-indigo-600 rounded-md ">"Explore Our Expertise"</a>
               </div>
            </div>
         </div>
         <div class="flex items-center justify-center w-full lg:w-1/2">
            <div class=""><img alt="Banner Image" src="/assets/banner.webp" width="616" height="617" /></div>
         </div>
      </div>
    }
}
