use html5stuck_scraper::scrape_site;

fn main() {
    let mut args = std::env::args().skip(1);
    let start = args.next().unwrap();
    let output = args.next().unwrap();
    for x in scrape_site(&start) {
        let x = x.unwrap();
        let out_path = format!("{}/{}.json", output, x.num);
        let json = serde_json::to_string(&x).unwrap();
        println!("{}: {}", out_path, json);
        std::fs::write(out_path, json).unwrap();
    }
}
