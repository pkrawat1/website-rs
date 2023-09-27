use leptos::*;

#[server(SendEmail, "/api")]
pub async fn send_email(email: String) -> Result<(), ServerFnError> {
  Ok(())
}


#[component]
pub fn HireUsPage(cx: Scope) -> impl IntoView {
    let (email, set_email) = create_signal(cx, "".to_string());
    let (subject, set_subject) = create_signal(cx, "".to_string());

    view! {cx,
      <div class="py-8 lg:py-16 px-4 mx-auto max-w-screen-md">
          <h2 class="mb-4 text-4xl tracking-tight font-extrabold text-center text-gray-900 dark:text-white">Contact Us</h2>
          <p class="mb-8 lg:mb-16 font-light text-center text-gray-500 dark:text-gray-400 sm:text-xl">
            "Reach out to us for expert software solutions and consulting services tailored to your unique needs. We're here to bring your vision to life."
          </p>
          <form action="#" class="space-y-8">
              <div>
                  <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">Your email</label>
                  <input
                      type="email"
                      id="email"
                      class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-indigo-500 focus:border-indigo-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-indigo-500 dark:focus:border-indigo-500 dark:shadow-sm-light"
                      placeholder="your@email.com"
                      required
                      on:input=move |ev| { set_email(event_target_value(&ev)); } 
                      props:value=email
                  />
              </div>
              <div>
                  <label for="subject" class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">Subject</label>
                  <input
                      type="text"
                      id="subject"
                      class="block p-3 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-indigo-500 dark:focus:border-indigo-500 dark:shadow-sm-light"
                      placeholder="Let us know how we can help you"
                      required
                      on:input=move |ev| { set_subject(event_target_value(&ev)); } 
                      props:value=subject
                  />
              </div>
              <div class="sm:col-span-2">
                  <label for="message" class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-400">Your message</label>
                  <textarea
                      id="message"
                      rows="6"
                      class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg shadow-sm border border-gray-300 focus:ring-indigo-500 focus:border-indigo-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-indigo-500 dark:focus:border-indigo-500"
                      placeholder="Leave a comment..."
                  ></textarea>
              </div>
              <button
                  type="submit"
                  on:click=move |_| {
                    spawn_local(async {
                        let _ = send_email("test@example.com".to_string()).await;
                    });
                  }
                  class="py-3 px-5 text-sm font-medium text-center text-white rounded-lg bg-indigo-700 sm:w-fit hover:bg-indigo-800 focus:ring-4 focus:outline-none focus:ring-indigo-300 dark:bg-indigo-600 dark:hover:bg-indigo-700 dark:focus:ring-indigo-800"
              >
                  Send message
              </button>
          </form>
      </div>

    }
}
