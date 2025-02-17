use std::error::Error;

use scraper::{Html, Selector};
use surf::Client;

fn get_title_href(body: String) -> Result<Vec<(String, String)>, Box<dyn Error>> {
	let document = Html::parse_document(&body);

	let div_select = Selector::parse("div.home-truyendecu").unwrap();
	let a_select = Selector::parse("a").unwrap();

	let mut result = Vec::new();

	document.select(&div_select).for_each(|div| {
		div.select(&a_select).for_each(|a| {
			let href = a.value().attr("href").unwrap();
			let title = a.value().attr("title").unwrap();

			result.push((title.to_string(), href.to_string()));
		});
	});
	Ok(result)
}

pub async fn get_latest_ln(
	client: &Client, page: i32,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
	let path = match page {
		1 => String::from("/latest"),
		_ => format!("/latest/page/{}", page),
	};

	let mut res = client.get(path).await?;
	let res_body = res.body_string().await?;

	get_title_href(res_body)
}

pub async fn get_completed_ln(
	client: &Client, page: i32,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
	let path = match page {
		1 => String::from("/completed"),
		_ => format!("/completed/page/{}", page),
	};

	let mut res = client.get(path).await?;
	let res_body = res.body_string().await?;

	get_title_href(res_body)
}

pub async fn get_genre_ln(
	client: &Client, genre: &String, page: i32,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
	let path = match page {
		1 => format!("/{}", genre),
		_ => format!("/{}/page/{}", genre, page),
	};

	let mut res = client.get(path).await?;
	let res_body = res.body_string().await?;

	get_title_href(res_body)
}

pub async fn get_title_ln(
	client: &Client, title: &String, page: i32,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
	let path = match page {
		1 => format!("/?s={}", title),
		_ => format!("/page/{}?s={}", page, title),
	};

	let mut res = client.get(path).await?;
	let res_body = res.body_string().await?;

	get_title_href(res_body)
}
