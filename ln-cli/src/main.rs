use std::error::Error;

use ln_lib::{LightnovelCategory, LightnovelList};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let mut list = LightnovelList::new(LightnovelCategory::Latest)?;
	list.scrape().await?;
	let mut ln = list.get_lightnovel(1);

	ln.scrape_chapter_page(None).await?;

	Ok(())
}
