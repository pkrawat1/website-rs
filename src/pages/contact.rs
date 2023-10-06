use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
struct Enquiry {
  name: String,
  email: String,
  source: String,
  body: String
}

#[server(SendEmail, "/api")]
pub async fn send_email(enquiry: Enquiry) -> Result<(), ServerFnError> {
  use dotenv;
  use reqwest::Client;
  use reqwest::StatusCode;
  use reqwest::header;
  use serde_json::json;
  use std::env;

  dotenv::dotenv().ok();
  
  if !(enquiry.name != "" && enquiry.email != "" && enquiry.source != "" && enquiry.body != "") {
    return Err(ServerFnError::ServerError("Please check all the required fields !".to_string()));
  }

    // Retrieve the API key as a String
    let api_key_str = env::var("BREVO_API_KEY").unwrap().to_string();

    // Convert the API key into a HeaderValue
    let api_key_header = match header::HeaderValue::from_str(&api_key_str) {
        Ok(header) => header,
        Err(err) => {
            eprintln!("Invalid API key: {}", err);
            return Err(ServerFnError::ServerError(err.to_string()));
        }
    };


    let body = json!(
        {
          "sender": {
            "name": env::var("SENDER_NAME").unwrap().to_string(),
            "email": env::var("SENDER_EMAIL").unwrap().to_string()
          },
          "to": [
            {
              "name": env::var("RECIPIENT_NAME").unwrap().to_string(),
              "email": env::var("RECIPIENT_EMAIL").unwrap().to_string()
            }
          ],
          "templateId": 8,
          "params": {
            "name": enquiry.name,
            "email": enquiry.email,
            "source": enquiry.source,
            "body": enquiry.body
          }
        }
    );

    let client = Client::new()
        .post("https://api.brevo.com/v3/smtp/email")
        .json(&body)
        .header(header::HeaderName::from_static("api-key"), api_key_header)
        .header(
            header::CONTENT_TYPE, 
            header::HeaderValue::from_static("application/json")
        );

    let response = client.send().await?;

    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => println!("Email sent!"),
        _ => eprintln!(
            "Unable to send your email. Status code was: {}. Body content was: {:?}",
            response.status(),
            response.text().await?
        ),
    }

    Ok(())
}

#[component]
pub fn HireUsPage(cx: Scope) -> impl IntoView {
    let (enquiry, set_enquiry) = create_signal(cx, Enquiry {..Default::default()});
    let (error, set_error) = create_signal(cx, "".to_string());

    view! {cx,
      <div class="py-8 lg:py-16 px-4 mx-auto max-w-screen-md">
          <h2 class="mb-4 text-4xl tracking-tight font-extrabold text-center text-gray-900 dark:text-white">Contact Us</h2>
          <p class="mb-8 lg:mb-16 font-light text-center text-gray-500 dark:text-gray-400 sm:text-xl">
            "Reach out to us for expert software solutions and consulting services tailored to your unique needs. We're here to bring your vision to life."
          </p>
          <Show
            when=move || { error() != "" }
            fallback=|_| ""
          >
            <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 mb-5 rounded relative" role="alert">
              <strong class="font-bold">"Holy smokes! "</strong>
              <span class="block sm:inline">{error}</span>
            </div>
          </Show>
          <form action="#" class="space-y-8">
              <div>
                  <label for="name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">
                    "Name"
                  </label>
                  <input
                      type="text"
                      id="name"
                      class="block p-3 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 shadow-sm focus:ring-indigo-500 focus:border-indigo-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-indigo-500 dark:focus:border-indigo-500 dark:shadow-sm-light"
                      placeholder="Full Name"
                      required
                      on:input=move |ev| {
                        set_enquiry(Enquiry {
                          name: event_target_value(&ev),
                          ..enquiry.get()
                        });
                      } 
                      props:value=move || {enquiry.get().name}
                  />
              </div>
              <div>
                  <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">
                    "Your email"
                  </label>
                  <input
                      type="email"
                      id="email"
                      class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-indigo-500 focus:border-indigo-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-indigo-500 dark:focus:border-indigo-500 dark:shadow-sm-light"
                      placeholder="your@email.com"
                      required
                      on:input=move |ev| {
                        set_enquiry(Enquiry {
                          email: event_target_value(&ev),
                          ..enquiry.get()
                        });
                      } 
                      props:value=move || {enquiry.get().email}
                  />
              </div>
              <div>
                  <label for="source" class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">
                    "Where did you find us?"
                  </label>
                  <select id="source" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                      on:input=move |ev| {
                        set_enquiry(Enquiry {
                          source: event_target_value(&ev),
                          ..enquiry.get()
                        });
                      } 
                      props:value=move || {enquiry.get().source}
                  >
                    <option selected>Where did you find us?</option>
                    <option>"Elixir Forum"</option>
                    <option>"Medium"</option>
                    <option>"Google"</option>
                    <option>"Github"</option>
                    <option>"LinkedIn"</option>
                  </select>
              </div>
              <div class="sm:col-span-2">
                  <label for="message" class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-400">
                    "Your message"
                  </label>
                  <textarea
                      id="message"
                      rows="6"
                      class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg shadow-sm border border-gray-300 focus:ring-indigo-500 focus:border-indigo-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-indigo-500 dark:focus:border-indigo-500"
                      placeholder="Leave a comment..."
                      on:input=move |ev| {
                        set_enquiry(Enquiry {
                          body: event_target_value(&ev),
                          ..enquiry.get()
                        });
                      } 
                      props:value=move || {enquiry.get().body}
                  ></textarea>
              </div>
              <button
                  type="submit"
                  on:click=move |_| {
                    spawn_local(async move {
                        match send_email(enquiry.get()).await {
                          Ok(_) => (),
                          Err(ServerFnError::ServerError(err)) => set_error(err),
                          Err(_) => ()
                        }
                    });
                  }
                  class="py-3 px-5 text-sm font-medium text-center text-white rounded-lg bg-indigo-700 sm:w-fit hover:bg-indigo-800 focus:ring-4 focus:outline-none focus:ring-indigo-300 dark:bg-indigo-600 dark:hover:bg-indigo-700 dark:focus:ring-indigo-800"
              >
                  "Send message"
              </button>
          </form>
      </div>

    }
}
