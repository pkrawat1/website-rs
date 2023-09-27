use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
      <Banner />
      <TrustedBy />
      <Benifits />
      <HighlightBenifits />
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

#[component]
fn TrustedBy(cx: Scope) -> impl IntoView {
    view! {cx,
      <div class="container mx-auto flex flex-col justify-center">
       <div class="-mb-5 mt-5 md:mb-0 text-xl text-center text-gray-700 dark:text-white">
          "Trusted by customers worldwide"
       </div>
       <div class="flex flex-wrap justify-center mt-5 md:mt-0 gap-5 md:justify-around items-center">
          <div class="text-gray-400 dark:text-gray-400 w-1/3 md:w-auto">
            <img alt="Reevoy logo" src="/assets/bitclass.webp"  width="200" />
          </div>
          <div class="text-gray-400 dark:text-gray-400 w-1/3 md:w-auto">
            <img alt="Reevoy logo" src="/assets/loantap.svg"  width="200" />
          </div>
          <div class="text-gray-400 dark:text-gray-400 w-1/3 md:w-auto">
            <img alt="Reevoy logo" src="/assets/qoala.png"  width="200" />
          </div>
          <div class="text-gray-400 dark:text-gray-400 w-1/3 md:w-auto">
            <img alt="Reevoy logo" src="/assets/martide.png"  width="200" />
          </div>
          <div class="mt-9 md:mt-0 text-gray-400 dark:text-gray-400 w-1/3 md:w-auto">
            <img alt="Reevoy logo" src="/assets/reevoy.png" width="200" />
          </div>
       </div>
      </div>
    }
}

#[component]
fn Benifits(cx: Scope) -> impl IntoView {
    view! {cx,
      <div class="container p-8 mx-auto xl:px-0 flex w-full flex-col mt-4 items-center justify-center text-center">
        <div class="text-sm font-bold tracking-wider text-indigo-600 uppercase">
          "Aviabird Benifits"
        </div>
        <h2 class="max-w-2xl mt-3 text-3xl font-bold leading-snug tracking-tight text-gray-800 lg:leading-tight lg:text-4xl dark:text-white">
          "Expertise Across Industries"
        </h2>
        <p class="max-w-2xl py-4 text-lg leading-normal text-gray-500 lg:text-xl xl:text-xl dark:text-gray-300">
          "With 8+ years of product development experience, Aviabird excels in building scalable solutions across diverse sectors, including InsureTech, FinTech, EduTech, Ecommerce, and Data Science."
        </p>
      </div>
    }
}

#[component]
fn HighlightBenifits(cx: Scope) -> impl IntoView {
    view! {cx,
      <div class="container p-8 mx-auto xl:px-0 flex flex-wrap mb-20 lg:gap-10 lg:flex-nowrap ">
        <div class="flex items-center justify-center w-full lg:w-1/2 ">
          <div><img alt="Benefits" width="521" height="548" src="assets/highlight.webp" /></div>
        </div>
        <div class="flex flex-wrap items-center w-full lg:w-1/2 ">
          <div>
            <div class="flex flex-col w-full mt-4">
              <h3 class="max-w-2xl mt-3 text-3xl font-bold leading-snug tracking-tight text-gray-800 lg:leading-tight lg:text-4xl dark:text-white">
                "Why Choose Aviabird?"
              </h3>
              <p class="max-w-2xl py-4 text-lg leading-normal text-gray-500 lg:text-xl xl:text-xl dark:text-gray-300">
                "Aviabird brings expertise in cutting-edge technologies, a proven track record in building scalable solutions, and a client-centric approach. With 8+ years of experience, we excel in delivering tailored consulting and development services across various industries."
              </p>
            </div>
            <div class="w-full mt-5">
              <div class="flex items-start mt-8 space-x-3">
                <div class="flex items-center justify-center flex-shrink-0 mt-1 bg-indigo-500 rounded-md w-11 h-11 ">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true" class="w-7 h-7 text-indigo-50">
                    <path fill-rule="evenodd" d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25zm-2.625 6c-.54 0-.828.419-.936.634a1.96 1.96 0 00-.189.866c0 .298.059.605.189.866.108.215.395.634.936.634.54 0 .828-.419.936-.634.13-.26.189-.568.189-.866 0-.298-.059-.605-.189-.866-.108-.215-.395-.634-.936-.634zm4.314.634c.108-.215.395-.634.936-.634.54 0 .828.419.936.634.13.26.189.568.189.866 0 .298-.059.605-.189.866-.108.215-.395.634-.936.634-.54 0-.828-.419-.936-.634a1.96 1.96 0 01-.189-.866c0-.298.059-.605.189-.866zm2.023 6.828a.75.75 0 10-1.06-1.06 3.75 3.75 0 01-5.304 0 .75.75 0 00-1.06 1.06 5.25 5.25 0 007.424 0z" clip-rule="evenodd"></path>
                  </svg>
                </div>
                <div>
                  <h4 class="text-xl font-medium text-gray-800 dark:text-gray-200">"Extensive Tech Expertise"</h4>
                  <p class="mt-1 text-gray-500 dark:text-gray-400">
                    "Proficiency in Elixir Phoenix, Go, Rust, React, Angular, and more."
                  </p>
                  <p class="mt-1 text-gray-500 dark:text-gray-400">
                    "Deep knowledge spanning InsureTech, FinTech, EduTech, Ecommerce, and Data Science."
                  </p>
                </div>
              </div>
              <div class="flex items-start mt-8 space-x-3">
                <div class="flex items-center justify-center flex-shrink-0 mt-1 bg-indigo-500 rounded-md w-11 h-11 ">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true" class="w-7 h-7 text-indigo-50">
                    <path fill-rule="evenodd" d="M3 6a3 3 0 013-3h12a3 3 0 013 3v12a3 3 0 01-3 3H6a3 3 0 01-3-3V6zm4.5 7.5a.75.75 0 01.75.75v2.25a.75.75 0 01-1.5 0v-2.25a.75.75 0 01.75-.75zm3.75-1.5a.75.75 0 00-1.5 0v4.5a.75.75 0 001.5 0V12zm2.25-3a.75.75 0 01.75.75v6.75a.75.75 0 01-1.5 0V9.75A.75.75 0 0113.5 9zm3.75-1.5a.75.75 0 00-1.5 0v9a.75.75 0 001.5 0v-9z" clip-rule="evenodd"></path>
                  </svg>
                </div>
                <div>
                  <h4 class="text-xl font-medium text-gray-800 dark:text-gray-200">"Proven Scalability"</h4>
                  <p class="mt-1 text-gray-500 dark:text-gray-400">
                    "Years of product development experience."
                  </p>
                  <p class="mt-1 text-gray-500 dark:text-gray-400">
                    "Track record of creating truly scalable solutions."
                  </p>
                </div>
              </div>
              <div class="flex items-start mt-8 space-x-3">
                <div class="flex items-center justify-center flex-shrink-0 mt-1 bg-indigo-500 rounded-md w-11 h-11 ">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true" class="w-7 h-7 text-indigo-50">
                    <path fill-rule="evenodd" d="M12 1.5a.75.75 0 01.75.75V4.5a.75.75 0 01-1.5 0V2.25A.75.75 0 0112 1.5zM5.636 4.136a.75.75 0 011.06 0l1.592 1.591a.75.75 0 01-1.061 1.06l-1.591-1.59a.75.75 0 010-1.061zm12.728 0a.75.75 0 010 1.06l-1.591 1.592a.75.75 0 01-1.06-1.061l1.59-1.591a.75.75 0 011.061 0zm-6.816 4.496a.75.75 0 01.82.311l5.228 7.917a.75.75 0 01-.777 1.148l-2.097-.43 1.045 3.9a.75.75 0 01-1.45.388l-1.044-3.899-1.601 1.42a.75.75 0 01-1.247-.606l.569-9.47a.75.75 0 01.554-.68zM3 10.5a.75.75 0 01.75-.75H6a.75.75 0 010 1.5H3.75A.75.75 0 013 10.5zm14.25 0a.75.75 0 01.75-.75h2.25a.75.75 0 010 1.5H18a.75.75 0 01-.75-.75zm-8.962 3.712a.75.75 0 010 1.061l-1.591 1.591a.75.75 0 11-1.061-1.06l1.591-1.592a.75.75 0 011.06 0z" clip-rule="evenodd"></path>
                  </svg>
                </div>
                <div>
                  <h4 class="text-xl font-medium text-gray-800 dark:text-gray-200">"Client-Centric Approach"</h4>
                  <p class="mt-1 text-gray-500 dark:text-gray-400">
                    "Tailored services encompassing consulting, development, training."
                  </p>
                  <p class="mt-1 text-gray-500 dark:text-gray-400">
                    "A steadfast commitment to understanding and fulfilling client needs."
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    }
}
