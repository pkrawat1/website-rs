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
    let load_feeds = create_resource(cx, || (), |_| async move { 
      match blog_feeds(None).await {
        Ok(blog_feeds) => blog_feeds,
        Err(_) => vec![]
      }
    });

    view! {cx,
      <section class="">
        <div class="py-8 px-4 mx-auto max-w-screen-xl lg:py-16 lg:px-6">
            <div class="mx-auto max-w-screen-sm text-center lg:mb-16 mb-8">
                <h2 class="mb-4 text-3xl lg:text-4xl tracking-tight font-extrabold text-gray-900 dark:text-white">"Our Blog"</h2>
                <p class="font-light text-gray-500 sm:text-xl dark:text-gray-400">"Innovative Tech Perspectives: Discover the Latest Insights and Expertise from Aviabird's Tech Blog Hub."</p>
            </div>
            <Transition
                fallback=move || view! {cx, 
                  <div class="w-full flex justify-center">
                    <span class="animate-ping absolute inline-flex h-7 w-7 rounded-full bg-sky-400 opacity-75"></span>
                  </div>
                }
            >
                <div class="grid gap-8 lg:grid-cols-2">
                    {
                        move || {
                          load_feeds
                            .read(cx)
                            .unwrap_or(vec![])
                            .into_iter()
                            .map(|feed| view! {cx,
                              <FeedItem feed=feed />
                            })
                            .collect_view(cx)
                        }
                    }
                </div>
            </Transition>
        </div>
      </section>
    }
}

#[component]
fn FeedItem(cx: Scope, feed: Feed) -> impl IntoView {
  use chrono::{NaiveDateTime};

  let parsed_date = NaiveDateTime::parse_from_str(&feed.pubDate, "%Y-%m-%d %H:%M:%S")
      .expect("Failed to parse date string");

  let date_time = chrono::DateTime::<chrono::Utc>::from_utc(parsed_date, chrono::Utc);
  let formatted_date = date_time.format("%d %b").to_string();

  
  view! {cx,
    <article class="p-6 bg-white rounded-lg border border-gray-200 shadow-md dark:bg-gray-800 dark:border-gray-700">
        <div class="flex justify-between items-center mb-5 text-gray-500">
            <span class="bg-indigo-100 text-indigo-800 text-xs font-medium inline-flex items-center px-2.5 py-0.5 rounded dark:bg-indigo-200 dark:text-indigo-800">
                <i class="fa-regular fa-newspaper fa-xl pr-2" />
                "Article"
            </span>
            <span class="text-sm">{formatted_date}</span>
        </div>
        <h2 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white"><a href="#">{feed.title}</a></h2>
        <p class="mb-5 font-light text-gray-500 dark:text-gray-400" inner_html=blog_description(feed.description)></p>
        <div class="flex justify-between items-center">
            <div class="flex items-center space-x-4">
                <img class="w-10 h-10 rounded-full" src=format!("https://api.multiavatar.com/{}.svg", feed.author.clone()) alt=feed.author.clone() />
                <span class="font-medium dark:text-white">
                  {feed.author}
                </span>
            </div>
            <a href=feed.link class="inline-flex items-center font-medium text-indigo-600 dark:text-indigo-500 hover:underline">
                "Read more"
                <i class="fa-solid fa-arrow-right ml-2"></i>
            </a>
        </div>
    </article>
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
