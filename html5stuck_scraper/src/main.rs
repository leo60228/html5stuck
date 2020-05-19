use html5stuck_scraper::scrape_site;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let mut args = std::env::args().skip(1);
    let start = args.next().unwrap();
    let output = args.next().unwrap();
    std::fs::create_dir_all(&output).unwrap();
    let len = 8054;
    let pb = ProgressBar::new(len);
    pb.set_style(ProgressStyle::default_bar().template("{msg} {wide_bar} {pos}/{len}"));
    pb.set_message("Scraping site");
    pb.set_draw_delta(len / 50);
    for x in pb.wrap_iter(scrape_site(Some(&output), &start)) {
        let (html, x) = x.unwrap();
        let out_path = format!("{}/{}.json", output, x.num);
        let html_path = format!("{}/{}.html", output, x.num);
        let json = serde_json::to_string(&x).unwrap();
        std::fs::write(out_path, json).unwrap();
        std::fs::write(html_path, html).unwrap();
    }
    pb.finish();
}
