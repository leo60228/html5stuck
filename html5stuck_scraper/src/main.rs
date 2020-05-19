use html5stuck_scraper::scrape_site;

fn main() {
    let mut args = std::env::args().skip(1);
    let start = args.next().unwrap();
    let output = args.next().unwrap();
    std::fs::create_dir_all(&output).unwrap();
    for x in scrape_site(Some(&output), &start) {
        let (html, x) = x.unwrap();
        let out_path = format!("{}/{}.json", output, x.num);
        let html_path = format!("{}/{}.html", output, x.num);
        let json = serde_json::to_string(&x).unwrap();
        println!("{}: {}", out_path, json);
        std::fs::write(out_path, json).unwrap();
        std::fs::write(html_path, html).unwrap();
    }
}
