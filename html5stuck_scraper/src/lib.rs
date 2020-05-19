use anyhow::{anyhow, Error, Result};
use html5stuck_common::{Page, Pesterlog, Theme};
use once_cell::sync::Lazy;
use retry::{delay::Fixed, retry};
use scraper::{Html, Selector};
use std::result::Result as StdResult;
use url::Url;

pub fn scrape_page(html: &str) -> Result<Page> {
    let doc = Html::parse_document(html);
    static CONSTANTS: Lazy<(
        Url,
        Selector,
        Selector,
        Selector,
        Selector,
        Selector,
        Selector,
        Selector,
        Selector,
        Selector,
    )> = Lazy::new(|| {
        let base_url = Url::parse("https://homestuck.com/story/1").unwrap();
        let title_sel = Selector::parse("h2.type-hs-header").unwrap();
        let contents_sel = Selector::parse(".o-story_text").unwrap();
        let pesterlog_type_sel = Selector::parse(".o_chat-log-btn").unwrap();
        let pesterlog_contents_sel = Selector::parse(".o_chat-log").unwrap();
        let next_sel = Selector::parse(".o_story-nav a").unwrap();
        let combo_img_sel = Selector::parse(r#"img[alt="Act6act5act1x2combo"]"#).unwrap();
        let combo_sel = Selector::parse("td.bg-hs-gray:last-child").unwrap();
        let canonical_sel = Selector::parse(r#"link[rel="canonical"]"#).unwrap();
        let back_sel =
            Selector::parse("ul:not(.o_game-options) > li.o_game-nav-item > a:not(#o_start-over)")
                .unwrap();
        (
            base_url,
            title_sel,
            contents_sel,
            pesterlog_type_sel,
            pesterlog_contents_sel,
            next_sel,
            combo_img_sel,
            combo_sel,
            canonical_sel,
            back_sel,
        )
    });
    let (
        base_url,
        title_sel,
        contents_sel,
        pesterlog_type_sel,
        pesterlog_contents_sel,
        next_sel,
        combo_img_sel,
        combo_sel,
        canonical_sel,
        back_sel,
    ) = &*CONSTANTS;
    let num: usize = doc
        .select(&canonical_sel)
        .next()
        .and_then(|x| x.value().attr("href"))
        .and_then(|x| x.rsplit('/').next())
        .ok_or_else(|| anyhow!("Missing canonical link!"))?
        .parse()?;
    let story_prev = if let Some(elem) = doc.select(&back_sel).last() {
        Some(
            base_url.join(
                elem.value()
                    .attr("href")
                    .ok_or_else(|| anyhow!("Invalid <a>!"))?,
            )?,
        )
    } else {
        None
    };
    let title = doc.select(&title_sel).next().map(|x| x.inner_html());
    let contents = doc.select(&contents_sel).next().map(|x| x.inner_html());
    let pesterlog_type = doc
        .select(&pesterlog_type_sel)
        .next()
        .map(|x| x.inner_html().trim_start_matches("Show ").to_string());
    let pesterlog_contents = doc
        .select(&pesterlog_contents_sel)
        .next()
        .map(|x| x.inner_html());
    let pesterlog = if let (Some(contents), Some(typ)) = (pesterlog_contents, pesterlog_type) {
        Some(Pesterlog { typ, contents })
    } else {
        None
    };
    let theme = if doc.select(&combo_img_sel).next().is_some() {
        let combo = doc
            .select(&combo_sel)
            .next()
            .ok_or_else(|| anyhow!("x2 combo missing second part!"))?;
        let pesterlog_type = combo
            .select(&pesterlog_type_sel)
            .next()
            .map(|x| x.inner_html().trim_start_matches("Show ").to_string());
        let pesterlog_contents = combo
            .select(&pesterlog_contents_sel)
            .next()
            .map(|x| x.inner_html());
        let pesterlog = if let (Some(contents), Some(typ)) = (pesterlog_contents, pesterlog_type) {
            Some(Pesterlog { typ, contents })
        } else {
            None
        };
        Theme::Combo { pesterlog }
    } else {
        Theme::Normal
    };
    let next = if let Some(elem) = doc.select(&next_sel).last() {
        base_url.join(
            elem.value()
                .attr("href")
                .ok_or_else(|| anyhow!("Invalid <a>!"))?,
        )?
    } else {
        let next = if num == 8130 {
            base_url.clone()
        } else {
            base_url.join(&format!("/story/{}", num + 1))?
        };
        let theme = if title.as_deref() == Some("[???????]") {
            Theme::Password
        } else {
            Theme::LinkInFlash
        };
        let contents = if theme == Theme::Password {
            None
        } else {
            contents
        };
        return Ok(Page {
            num,
            title,
            contents,
            pesterlog: None,
            story_next: None,
            story_prev,
            next,
            theme,
        });
    };
    let next_num: usize = next
        .path_segments()
        .and_then(|x| x.rev().next())
        .ok_or_else(|| anyhow!("Pathless link!"))?
        .parse()?;
    let (next, story_next) = if next_num <= num {
        let real_next = base_url.join(&format!("/story/{}", num + 1))?;
        (real_next, Some(next))
    } else {
        (next, None)
    };
    Ok(Page {
        num,
        title,
        contents,
        pesterlog,
        story_next,
        story_prev,
        next,
        theme,
    })
}

pub fn scrape_url(url: &str) -> Result<Page> {
    scrape_page(&attohttpc::get(url).send()?.text()?)
}

pub fn scrape_site(start: &str) -> impl Iterator<Item = StdResult<Page, retry::Error<Error>>> {
    let first_page = retry(Fixed::from_millis(15000).take(3), || scrape_url(start));
    std::iter::successors(Some(first_page), |last| {
        if let Ok(x) = last {
            Some(retry(Fixed::from_millis(15000).take(3), || {
                scrape_url(x.next.as_str())
            }))
        } else {
            None
        }
    })
}
