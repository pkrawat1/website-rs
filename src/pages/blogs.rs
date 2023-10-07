use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Feed {
  author: String,
  description: String,
  pubDate: String,
  title: String,
  link: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlogFeedsResponse {
  items: Vec<Feed>
}

#[server(BlogFeeds, "/api", "Cbor")]
pub async fn blog_feeds(tag: Option<String>) -> Result<Vec<Feed>, ServerFnError> {
  use reqwest::Client;
  use reqwest::StatusCode;

  let url = "https://api.rss2json.com/v1/api.json?rss_url=https://medium.com/feed/aviabird";
  let url = if let Some(tag) = tag {
    url.to_owned() + &tag
  } else {
    url.to_string()
  };

  let client = Client::new()
      .get(url);
      

    let response = client.send().await?;

    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => 
          Ok(response.json::<BlogFeedsResponse>().await?.items)
       ,
        _ => 
          Err(ServerFnError::ServerError(String::from("Unable to load blog feeds")))
        
    }
}

#[component]
pub fn BlogsPage(cx: Scope) -> impl IntoView {
    let (feeds, set_feeds) = create_signal(cx, vec![]);
    view! {cx, 
      <div>
        Total
        <button 
          on:click=move |_| {
            spawn_local(async move {
                  match blog_feeds(None).await {
                    Ok(blog_feeds) => set_feeds(blog_feeds),
                    Err(_) => set_feeds(vec![])
                  }
                });
          }
        >
          Feeds
        </button>
      </div>
      <section class="">
        <div class="py-8 px-4 mx-auto max-w-screen-xl lg:py-16 lg:px-6">
            <div class="mx-auto max-w-screen-sm text-center lg:mb-16 mb-8">
                <h2 class="mb-4 text-3xl lg:text-4xl tracking-tight font-extrabold text-gray-900 dark:text-white">Our Blog</h2>
                <p class="font-light text-gray-500 sm:text-xl dark:text-gray-400">We use an agile approach to test assumptions and connect with the needs of your audience early and often.</p>
            </div> 
            <div class="grid gap-8 lg:grid-cols-2">
                {
                  move || { feeds.get().into_iter()
                    .map(|feed| view! {cx,
                      <article class="p-6 bg-white rounded-lg border border-gray-200 shadow-md dark:bg-gray-800 dark:border-gray-700">
                          <div class="flex justify-between items-center mb-5 text-gray-500">
                              <span class="bg-primary-100 text-primary-800 text-xs font-medium inline-flex items-center px-2.5 py-0.5 rounded dark:bg-primary-200 dark:text-primary-800">
                                  <svg class="mr-1 w-3 h-3" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path d="M2 6a2 2 0 012-2h6a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V6zM14.553 7.106A1 1 0 0014 8v4a1 1 0 00.553.894l2 1A1 1 0 0018 13V7a1 1 0 00-1.447-.894l-2 1z"></path></svg>
                                  "Article"
                              </span>
                              <span class="text-sm">{feed.pubDate}</span>
                          </div>
                          <h2 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white"><a href="#">{feed.title}</a></h2>
                          <p class="mb-5 font-light text-gray-500 dark:text-gray-400" inner_html=blog_description(feed.description)></p>
                          <div class="flex justify-between items-center">
                              <div class="flex items-center space-x-4">
                                  <img class="w-7 h-7 rounded-full" src="https://flowbite.s3.amazonaws.com/blocks/marketing-ui/avatars/jese-leos.png" alt="Jese Leos avatar" />
                                  <span class="font-medium dark:text-white">
                                    {feed.author}
                                  </span>
                              </div>
                              <a href=feed.link class="inline-flex items-center font-medium text-primary-600 dark:text-primary-500 hover:underline">
                                  Read more
                                  <svg class="ml-2 w-4 h-4" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                              </a>
                          </div>
                      </article> 
                    })
                    .collect_view(cx) }
                }
            </div>  
        </div>
      </section>
    }
}

fn blog_description(description: String) -> String {
  // Find the first occurrence of "<p>"
    if let Some(start_idx) = description.find("<p>") {
        // Find the closing tag "</p>" after the "<p>"
        if let Some(end_idx) = description[start_idx..].find("</p>") {
            // Extract the text between "<p>" and "</p>"
            let matched_text = &description[start_idx + 3..start_idx + end_idx];

            // Truncate the extracted text to the specified length
            return matched_text.chars().take(250).collect::<String>();
        }
    }
    description
}
