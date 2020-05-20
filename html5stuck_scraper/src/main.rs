use html5stuck_scraper::{scrape_page, scrape_site};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn linked_list(start: &str, output: &str) {
    let len = 8054;
    let pb = ProgressBar::new(len);
    pb.set_style(ProgressStyle::default_bar().template("{msg} {wide_bar} {pos}/{len}"));
    pb.set_message("Downloading and scraping site");
    for x in pb.wrap_iter(scrape_site(Some(output), start)) {
        let (html, x) = x.unwrap();
        let out_path = format!("{}/{}.json", output, x.num);
        let html_path = format!("{}/{}.html", output, x.num);
        let json = serde_json::to_string(&x).unwrap();
        std::fs::write(out_path, json).unwrap();
        std::fs::write(html_path, html).unwrap();
    }
    pb.finish();
}

fn freshen(output: &str) {
    let files: Vec<_> = std::fs::read_dir(output).unwrap().collect();
    let len = 8054;
    let pb = ProgressBar::new(len);
    pb.set_style(ProgressStyle::default_bar().template("{msg} {wide_bar} {pos}/{len}"));
    pb.set_message("Scraping site");
    pb.set_draw_delta(len / 50);
    files
        .into_par_iter()
        .map(Result::unwrap)
        .filter(|x| x.path().extension().map(|x| x == "html").unwrap_or(false))
        .progress_with(pb.clone())
        .for_each(|file| {
            let html = std::fs::read_to_string(&file.path()).unwrap();
            let x = scrape_page(&html).unwrap();
            let out_path = format!("{}/{}.json", output, x.num);
            let json = serde_json::to_string(&x).unwrap();
            std::fs::write(out_path, json).unwrap();
        });
    pb.finish();
}

fn main() {
    let mut args = std::env::args().skip(1);
    let start = args.next().unwrap();
    let output = args.next().unwrap();
    let new = args.next().as_deref() == Some("--new");
    if new && std::path::Path::new(&output).exists() {
        return;
    }
    std::fs::create_dir_all(&output).unwrap();
    if std::fs::read_dir(&output).unwrap().count() >= 16108 {
        freshen(&output);
    } else {
        linked_list(&start, &output);
    }
}
