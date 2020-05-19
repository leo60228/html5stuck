use anyhow::{anyhow, Context as _, Result};
use askama::Template;
use html5stuck_common::Page;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

mod filters {
    use url::Url;

    pub fn last_segment(url: &Url) -> askama::Result<&str> {
        Ok(url.path_segments().unwrap().rev().next().unwrap())
    }
}

#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate<'a> {
    page: &'a Page,
    next: &'a Page,
}

pub fn render_page(page: &Page, next: &Page) -> Result<String> {
    let template = PageTemplate { page, next };
    Ok(template.render()?)
}

fn parse_page_file(path: &Path) -> Result<Page> {
    let file = File::open(path).context("Opening file")?;
    let reader = BufReader::new(file);
    let page = serde_json::from_reader(reader).context("Parsing JSON")?;

    Ok(page)
}

pub fn batch_render(path: impl AsRef<Path>, output: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    let output = output.as_ref();
    fs::create_dir_all(output.join("story"))?;
    let files: Vec<_> = fs::read_dir(path)
        .context("Getting directory listing")?
        .collect();
    let len = files.len() as _;
    let pb = ProgressBar::new(len);
    pb.set_style(ProgressStyle::default_bar().template("{msg} {wide_bar} {pos}/{len}"));
    pb.set_message("Generating story pages");
    pb.set_draw_delta(len / 50);
    for file in pb.wrap_iter(files.into_iter()) {
        let file = file.context("Iterating over directory listing")?;
        let current = parse_page_file(&file.path()).context("Parsing current page")?;
        let next_num: usize = current
            .next
            .path_segments()
            .and_then(|x| x.rev().next())
            .ok_or_else(|| anyhow!("Pathless link!"))?
            .parse()?;
        let next = parse_page_file(&path.join(format!("{}.json", next_num)))
            .context("Parsing next page")?;
        let rendered = render_page(&current, &next).context("Rendering page")?;
        fs::write(
            &output.join(format!("story/{}.html", current.num)),
            &rendered,
        )
        .context("Writing rendered")?;
    }
    pb.finish();
    Ok(())
}
