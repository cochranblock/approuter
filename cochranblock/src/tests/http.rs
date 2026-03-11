// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use std::time::Instant;
use tempfile::TempDir;

use super::t24;
use super::assert_ok;

use crate::t0;
use crate::web::router;

async fn run(p0: &str, p1: impl std::future::Future<Output = Result<(), String>>) -> t24 {
    let v0 = Instant::now();
    match p1.await {
        Ok(()) => t24 { s30: p0.into(), s31: true, s32: v0.elapsed().as_millis() as u64, s33: None },
        Err(msg) => t24 { s30: p0.into(), s31: false, s32: v0.elapsed().as_millis() as u64, s33: Some(msg) },
    }
}

async fn server() -> (String, reqwest::Client, TempDir) {
    let v0 = TempDir::new().unwrap();
    let p0 = t0 {};
    let v1 = router::f1(p0);
    let v2 = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let v3 = v2.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(v2, v1.into_make_service()).await.unwrap();
    });
    let v4 = format!("http://{}", v3);
    let v5 = reqwest::Client::builder()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .build().unwrap();
    (v4, v5, v0)
}

pub async fn f51() -> Vec<t24> {
    let mut v0 = Vec::new();
    let (v1, v2, _) = server().await;

    v0.push(run("index_200", async {
        let v3 = v2.get(format!("{}/", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(status.is_success(), format!("status {} not 2xx", status))?;
        assert_ok(v4.contains("CochranBlock"), "home page missing 'CochranBlock'")?;
        Ok(())
    }).await);
    v0.push(run("services_200", async {
        let v3 = v2.get(format!("{}/services", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(status.is_success(), format!("status {}", status))?;
        assert_ok(v4.contains("What We Do") || v4.contains("Rust-only SaaS"), "services page missing content")?;
        Ok(())
    }).await);
    v0.push(run("about_200", async {
        let v3 = v2.get(format!("{}/about", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(status.is_success(), format!("status {}", status))?;
        assert_ok(v4.contains("About CochranBlock") || v4.contains("About"), "about page missing about content")?;
        assert_ok(v4.contains("Mission") || v4.contains("Credentials"), "about page missing Mission or Credentials")?;
        assert_ok(v4.contains("Credentials"), "about page missing 'Credentials'")?;
        Ok(())
    }).await);
    v0.push(run("contact_200", async {
        let v3 = v2.get(format!("{}/contact", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(status.is_success(), format!("status {}", status))?;
        assert_ok(v4.contains("Contact"), "contact page missing 'Contact'")?;
        Ok(())
    }).await);
    v0.push(run("products_200", async {
        let v3 = v2.get(format!("{}/products", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(status.is_success(), format!("status {}", status))?;
        assert_ok(v4.contains("Products"), "products page missing 'Products'")?;
        assert_ok(v4.contains("Coming Soon"), "products page missing 'Coming Soon'")?;
        assert_ok(v4.contains("Rogue Repo") || v4.contains("CochranBlock"), "products missing Rogue Repo")?;
        assert_ok(v4.contains("Ronin Sites"), "products missing Ronin Sites")?;
        assert_ok(v4.contains("Kova"), "products missing Kova")?;
        Ok(())
    }).await);
    v0.push(run("book_200", async {
        let v3 = v2.get(format!("{}/book", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(status.is_success(), format!("status {}", status))?;
        assert_ok(v4.contains("Schedule a Discovery Call"), "book page missing heading")?;
        assert_ok(v4.contains("Braintrust"), "book page missing Braintrust context")?;
        assert_ok(v4.contains("30 minutes"), "book page missing duration")?;
        Ok(())
    }).await);
    v0.push(run("book_calendar_structure", async {
        let v3 = v2.get(format!("{}/book", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("booking-grid"), "calendar missing booking grid")?;
        assert_ok(v4.contains("booking-month"), "calendar missing month label")?;
        assert_ok(v4.contains("booking-prev"), "calendar missing prev button")?;
        assert_ok(v4.contains("booking-next"), "calendar missing next button")?;
        assert_ok(v4.contains("booking-slots-data"), "calendar missing slots JSON")?;
        assert_ok(v4.contains("booking.js"), "calendar missing booking.js")?;
        assert_ok(v4.contains("booking-hint"), "calendar missing hint")?;
        Ok(())
    }).await);
    v0.push(run("book_slots_mailto", async {
        let v3 = v2.get(format!("{}/book", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("mailto:"), "book page missing mailto links")?;
        assert_ok(v4.contains("Discovery%20Call"), "mailto missing Discovery Call subject")?;
        Ok(())
    }).await);
    v0.push(run("robots_txt", async {
        let v3 = v2.get(format!("{}/robots.txt", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(status.is_success(), format!("robots.txt status {}", status))?;
        assert_ok(v4.contains("User-agent: *"), "robots.txt missing User-agent")?;
        assert_ok(v4.contains("Sitemap:"), "robots.txt missing Sitemap")?;
        Ok(())
    }).await);
    v0.push(run("sitemap_xml", async {
        let v3 = v2.get(format!("{}/sitemap.xml", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(status.is_success(), format!("sitemap.xml status {}", status))?;
        assert_ok(v4.contains("<urlset"), "sitemap missing urlset")?;
        assert_ok(v4.contains("cochranblock.org/"), "sitemap missing URLs")?;
        Ok(())
    }).await);
    v0.push(run("json_ld_org", async {
        let v3 = v2.get(format!("{}/", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("application/ld+json"), "home missing JSON-LD script")?;
        assert_ok(v4.contains("schema.org") && v4.contains("Organization"), "home missing Organization schema")?;
        Ok(())
    }).await);
    v0.push(run("health_ok", async {
        let v3 = v2.get(format!("{}/health", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(status.is_success(), format!("status {}", status))?;
        assert_ok(v4 == "OK", format!("health returned '{}' not 'OK'", v4))?;
        Ok(())
    }).await);
    v0.push(run("static_css", async {
        let v3 = v2.get(format!("{}/assets/css/main.css", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let ct = v3.headers().get("content-type").and_then(|h| h.to_str().ok()).unwrap_or("");
        assert_ok(status.is_success(), format!("status {}", status))?;
        assert_ok(ct.contains("css"), format!("content-type {} not css", ct))?;
        Ok(())
    }).await);
    v0.push(run("calendar_js", async {
        let v3 = v2.get(format!("{}/assets/js/calendar.js", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(status.is_success(), format!("status {}", status))?;
        assert_ok(v4.contains("parseSlots"), "calendar.js missing parseSlots")?;
        assert_ok(v4.contains("renderMonth"), "calendar.js missing renderMonth")?;
        assert_ok(v4.contains("renderWeek"), "calendar.js missing renderWeek")?;
        Ok(())
    }).await);
    v0.push(run("favicon_svg", async {
        let v3 = v2.get(format!("{}/assets/favicon.svg", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(status.is_success(), format!("status {}", status))?;
        assert_ok(v4.contains("<svg"), "favicon not valid SVG")?;
        Ok(())
    }).await);
    v0.push(run("resume_pdf", async {
        let v3 = v2.get(format!("{}/assets/resume.pdf", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let ct = v3.headers().get("content-type").and_then(|h| h.to_str().ok()).unwrap_or("");
        assert_ok(status.is_success(), format!("status {}", status))?;
        assert_ok(ct.contains("pdf"), format!("content-type {} not pdf", ct))?;
        Ok(())
    }).await);
    v0.push(run("index_business", async {
        let v3 = v2.get(format!("{}/", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(status.is_success(), format!("status {}", status))?;
        assert_ok(v4.contains("Rust") && v4.contains("SaaS"), "home missing Rust SaaS messaging")?;
        assert_ok(v4.contains("Get in Touch") || v4.contains("Book a Call"), "home missing CTA")?;
        Ok(())
    }).await);
    v0.push(run("404", async {
        let v3 = v2.get(format!("{}/nonexistent", v1)).send().await.map_err(|e| e.to_string())?;
        let status = v3.status();
        assert_ok(status.as_u16() == 404, format!("expected 404 got {}", status))?;
        Ok(())
    }).await);
    v0.push(run("services_content", async {
        let v3 = v2.get(format!("{}/services", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("Rust-only SaaS") || v4.contains("Rust"), "services missing product")?;
        assert_ok(v4.contains("Systems Engineering") || v4.contains("Consulting"), "services missing consulting")?;
        assert_ok(v4.contains("Get in Touch"), "services missing CTA")?;
        Ok(())
    }).await);
    v0.push(run("contact_links", async {
        let v3 = v2.get(format!("{}/contact", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("mailto:mclarkfyrue@gmail.com"), "contact missing email link")?;
        assert_ok(v4.contains("Book a Call"), "contact missing Book a Call")?;
        assert_ok(v4.contains("CochranBlock%20Inquiry") || v4.contains("mailto:"), "contact mailto missing")?;
        Ok(())
    }).await);
    v0.push(run("about_tabs", async {
        let v3 = v2.get(format!("{}/about", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("Mission") || v4.contains("Profile"), "about missing Mission tab")?;
        assert_ok(v4.contains("Credentials"), "about missing Credentials tab")?;
        assert_ok(v4.contains("Print Resume"), "about missing print button")?;
        assert_ok(v4.contains("resume-section"), "about missing resume section")?;
        Ok(())
    }).await);
    v0.push(run("home_ctas", async {
        let v3 = v2.get(format!("{}/", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("href=\"/contact\""), "home missing contact link")?;
        assert_ok(v4.contains("href=\"/book\""), "home missing book link")?;
        assert_ok(v4.contains("href=\"/about\""), "home missing about link")?;
        assert_ok(v4.contains("href=\"/services\""), "home missing services link")?;
        Ok(())
    }).await);
    v0.push(run("book_slots_json", async {
        let v3 = v2.get(format!("{}/book", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        let v5 = v4.find("id=\"booking-slots-data\"").ok_or("no slots element")?;
        let v6 = v4[v5..].find('>').ok_or("no content start")?;
        let v7_start = v5 + v6 + 1;
        let v7_end = v4[v7_start..].find("</script>").ok_or("no script end")? + v7_start;
        let v8 = v4[v7_start..v7_end].trim();
        let v9: Vec<serde_json::Value> = serde_json::from_str(v8).map_err(|e| e.to_string())?;
        assert_ok(!v9.is_empty(), "slots must not be empty")?;
        let v10 = &v9[0];
        let v11 = v10.get("date").and_then(|x| x.as_str()).ok_or("slot missing date")?;
        assert_ok(v11.len() >= 10, "date must be YYYY-MM-DD")?;
        let v12 = v10.get("times").and_then(|x| x.as_array()).ok_or("slot missing times")?;
        assert_ok(!v12.is_empty(), "slot must have times")?;
        let v13 = v12[0].get("mailto").and_then(|x| x.as_str()).ok_or("time missing mailto")?;
        assert_ok(v13.starts_with("mailto:"), "mailto must be valid")?;
        Ok(())
    }).await);
    v0.push(run("booking_js", async {
        let v3 = v2.get(format!("{}/assets/js/booking.js", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("parseSlots"), "booking.js missing parseSlots")?;
        assert_ok(v4.contains("renderMonth"), "booking.js missing renderMonth")?;
        assert_ok(v4.contains("booking-slots-data"), "booking.js missing slots data id")?;
        Ok(())
    }).await);
    v0.push(run("logo_svg", async {
        let v3 = v2.get(format!("{}/assets/cochranblock-logo.svg", v1)).send().await.map_err(|e| e.to_string())?;
        assert_ok(v3.status().is_success(), "logo must 200")?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("<svg"), "logo must be valid SVG")?;
        Ok(())
    }).await);
    v0.push(run("assets_404", async {
        let v3 = v2.get(format!("{}/assets/nonexistent.file", v1)).send().await.map_err(|e| e.to_string())?;
        assert_ok(v3.status().as_u16() == 404, format!("assets 404 expected, got {}", v3.status()))?;
        Ok(())
    }).await);
    v0.push(run("removed_routes_404", async {
        for path in ["/source", "/ai-orchestration", "/prompts", "/cursor-rules", "/executive-summary"] {
            let v3 = v2.get(format!("{}{}", v1, path)).send().await.map_err(|e| e.to_string())?;
            assert_ok(v3.status().as_u16() == 404, format!("{} must 404, got {}", path, v3.status()))?;
        }
        Ok(())
    }).await);
    #[cfg(feature = "dev")]
    v0.push(run("dev_routes_200", async {
        for path in ["/dev/source", "/dev/exec-summary", "/dev/rules", "/dev/ai-orchestration", "/dev/prompts"] {
            let v3 = v2.get(format!("{}{}", v1, path)).send().await.map_err(|e| e.to_string())?;
            assert_ok(v3.status().is_success(), format!("{} must 200, got {}", path, v3.status()))?;
        }
        Ok(())
    }).await);
    v0.push(run("nav_footer_links", async {
        let v3 = v2.get(format!("{}/", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("href=\"/services\""), "nav/footer missing /services")?;
        assert_ok(v4.contains("href=\"/products\""), "nav/footer missing /products")?;
        assert_ok(v4.contains("href=\"/about\""), "nav/footer missing /about")?;
        assert_ok(v4.contains("href=\"/contact\""), "nav/footer missing /contact")?;
        assert_ok(v4.contains("href=\"/book\""), "nav/footer missing /book")?;
        assert_ok(v4.contains("href=\"/federal-partners\""), "nav/footer missing /federal-partners")?;
        assert_ok(v4.contains("href=\"/\"") || v4.contains("href=\"https://cochranblock.org\""), "nav/footer missing home link")?;
        Ok(())
    }).await);
    v0.push(run("federal_partners_200", async {
        let v3 = v2.get(format!("{}/federal-partners", v1)).send().await.map_err(|e| e.to_string())?;
        assert_ok(v3.status().is_success(), format!("federal-partners must 200"))?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("FBI") && v4.contains("DOD"), "federal-partners must mention FBI and DOD")?;
        assert_ok(v4.contains("Inspector General") || v4.contains("IG"), "federal-partners must mention IG")?;
        assert_ok(v4.contains("COTS") || v4.contains("commercial off-the-shelf"), "federal-partners must mention COTS")?;
        Ok(())
    }).await);
    v0.push(run("gzip_encoding", async {
        let v3 = v2.get(format!("{}/", v1))
            .header("Accept-Encoding", "gzip")
            .send().await.map_err(|e| e.to_string())?;
        let enc = v3.headers().get("Content-Encoding").and_then(|h| h.to_str().ok()).unwrap_or("");
        assert_ok(enc.contains("gzip") || enc.is_empty(), format!("gzip or identity ok, got encoding: {}", enc))?;
        assert_ok(v3.status().is_success(), "must succeed")?;
        Ok(())
    }).await);
    v0.push(run("hero_product_status", async {
        let v3 = v2.get(format!("{}/", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("Product in development") || v4.contains("hero-status"), "hero must show product status")?;
        assert_ok(v4.contains("Consulting") && (v4.contains("limited") || v4.contains("capacity")), "hero must show consulting availability")?;
        Ok(())
    }).await);
    v0.push(run("products_all_coming_soon", async {
        let v3 = v2.get(format!("{}/products", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("Rogue Repo"), "products must list Rogue Repo")?;
        assert_ok(v4.contains("Ronin Sites"), "products must list Ronin Sites")?;
        assert_ok(v4.contains("Kova"), "products must list Kova")?;
        assert_ok(v4.contains("Coming Soon"), "products must show Coming Soon")?;
        assert_ok(v4.contains("Get Notified") || v4.contains("href=\"/contact\""), "products must have contact CTA")?;
        assert_ok(v4.contains("/assets/img/") && v4.contains(".png"), "products must have product images")?;
        Ok(())
    }).await);
    v0.push(run("contact_product_launch_cta", async {
        let v3 = v2.get(format!("{}/contact", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("Product Launch") || v4.contains("Product%20Launch"), "contact must have product launch CTA")?;
        Ok(())
    }).await);
    v0.push(run("about_founded_by", async {
        let v3 = v2.get(format!("{}/about", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("Michael Cochran") || v4.contains("Founded"), "about must have founder")?;
        Ok(())
    }).await);
    v0.push(run("services_consulting_open", async {
        let v3 = v2.get(format!("{}/services", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("Consulting") && (v4.contains("open") || v4.contains("limited")), "services must show consulting availability")?;
        Ok(())
    }).await);
    v0.push(run("semantic_main_nav", async {
        let v3 = v2.get(format!("{}/", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("<main") || v4.contains("id=\"main\""), "page must have main landmark")?;
        assert_ok(v4.contains("<nav") || v4.contains("class=\"nav\""), "page must have nav")?;
        assert_ok(v4.contains("skip-link") || v4.contains("Skip to main"), "page must have skip link for a11y")?;
        Ok(())
    }).await);
    v0.push(run("html_doctype", async {
        let v3 = v2.get(format!("{}/", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.trim_start().starts_with("<!DOCTYPE html>"), "page must have DOCTYPE")?;
        Ok(())
    }).await);
    v0.push(run("meta_viewport", async {
        let v3 = v2.get(format!("{}/", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("viewport") && v4.contains("width=device-width"), "page must have viewport meta for mobile")?;
        Ok(())
    }).await);
    v0.push(run("product_images_200", async {
        for path in ["/assets/img/rogue-repo.png", "/assets/img/ronin-sites.png", "/assets/img/kova.png"] {
            let v3 = v2.get(format!("{}{}", v1, path)).send().await.map_err(|e| e.to_string())?;
            assert_ok(v3.status().is_success(), format!("{} must 200", path))?;
            let ct = v3.headers().get("content-type").and_then(|h| h.to_str().ok()).unwrap_or("");
            assert_ok(ct.contains("png"), format!("{} must be png", path))?;
        }
        Ok(())
    }).await);
    v0.push(run("products_services_link", async {
        let v3 = v2.get(format!("{}/products", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("href=\"/services\""), "products must link to services")?;
        Ok(())
    }).await);
    v0.push(run("products_roguerepo_link", async {
        let v3 = v2.get(format!("{}/products", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("href=\"https://roguerepo.io\""), "products must link Rogue Repo to roguerepo.io")?;
        Ok(())
    }).await);
    v0.push(run("products_ronin_link", async {
        let v3 = v2.get(format!("{}/products", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("href=\"https://ronin-sites.pro\""), "products must link Ronin Sites to ronin-sites.pro")?;
        Ok(())
    }).await);
    v0.push(run("book_weekdays_only", async {
        let v3 = v2.get(format!("{}/book", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        let v5 = v4.find("id=\"booking-slots-data\"").ok_or("no slots")?;
        let v6 = v4[v5..].find('>').ok_or("no content")?;
        let v7_start = v5 + v6 + 1;
        let v7_end = v4[v7_start..].find("</script>").ok_or("no end")? + v7_start;
        let v8: Vec<serde_json::Value> = serde_json::from_str(v4[v7_start..v7_end].trim()).map_err(|e| e.to_string())?;
        assert_ok(!v8.is_empty(), "slots must not be empty")?;
        let v9 = v8[0].get("day_name").and_then(|x| x.as_str()).unwrap_or("");
        assert_ok(!v9.is_empty() && (v9.starts_with("Mon") || v9.starts_with("Tue") || v9.starts_with("Wed") || v9.starts_with("Thu") || v9.starts_with("Fri")), "slots must be weekdays only")?;
        Ok(())
    }).await);
    v0.push(run("services_product_details", async {
        let v3 = v2.get(format!("{}/services", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("Offline-first") || v4.contains("Offline"), "services must show offline-first")?;
        assert_ok(v4.contains("Creative mode") || v4.contains("creative"), "services must show creative mode")?;
        assert_ok(v4.contains("Superior pricing") || v4.contains("pricing"), "services must show pricing")?;
        Ok(())
    }).await);
    v0.push(run("services_consulting_details", async {
        let v3 = v2.get(format!("{}/services", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("Systems Engineering") || v4.contains("Systems"), "services must show systems eng")?;
        assert_ok(v4.contains("Vulnerability") || v4.contains("vuln") || v4.contains("security"), "services must show vuln research")?;
        assert_ok(v4.contains("API") || v4.contains("Integration"), "services must show API/integration")?;
        Ok(())
    }).await);
    v0.push(run("index_tagline", async {
        let v3 = v2.get(format!("{}/", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("big guys") || v4.contains("greed"), "home must have value prop tagline")?;
        Ok(())
    }).await);
    v0.push(run("contact_testimonial", async {
        let v3 = v2.get(format!("{}/contact", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("Delivered") || v4.contains("client") || v4.contains("testimonial"), "contact must have trust signal")?;
        Ok(())
    }).await);
    v0.push(run("buttons_nav_all_200", async {
        for (path, needle) in [("/", "CochranBlock"), ("/services", "What We Do"), ("/products", "Products"), ("/federal-partners", "FBI"), ("/about", "About"), ("/contact", "Contact"), ("/book", "Schedule")] {
            let v3 = v2.get(format!("{}{}", v1, path)).send().await.map_err(|e| e.to_string())?;
            assert_ok(v3.status().is_success(), format!("{} must 200", path))?;
            let v4 = v3.text().await.map_err(|e| e.to_string())?;
            assert_ok(v4.contains(needle), format!("{} missing expected content", path))?;
        }
        Ok(())
    }).await);
    v0.push(run("buttons_hero_ctas_200", async {
        let v3 = v2.get(format!("{}/", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("href=\"/services\""), "hero missing What We Build link")?;
        assert_ok(v4.contains("href=\"/book\""), "hero missing Book a Call link")?;
        assert_ok(v4.contains("href=\"/contact\""), "hero missing Get in Touch link")?;
        assert_ok(v4.contains("href=\"/about\""), "hero missing About link")?;
        let r_svc = v2.get(format!("{}/services", v1)).send().await.map_err(|e| e.to_string())?;
        let r_book = v2.get(format!("{}/book", v1)).send().await.map_err(|e| e.to_string())?;
        let r_contact = v2.get(format!("{}/contact", v1)).send().await.map_err(|e| e.to_string())?;
        let r_about = v2.get(format!("{}/about", v1)).send().await.map_err(|e| e.to_string())?;
        assert_ok(r_svc.status().is_success(), "hero /services link must 200")?;
        assert_ok(r_book.status().is_success(), "hero /book link must 200")?;
        assert_ok(r_contact.status().is_success(), "hero /contact link must 200")?;
        assert_ok(r_about.status().is_success(), "hero /about link must 200")?;
        Ok(())
    }).await);
    v0.push(run("buttons_footer_links_200", async {
        for path in ["/", "/services", "/products", "/federal-partners", "/about", "/contact", "/book"] {
            let v3 = v2.get(format!("{}{}", v1, path)).send().await.map_err(|e| e.to_string())?;
            assert_ok(v3.status().is_success(), format!("footer {} must 200", path))?;
        }
        Ok(())
    }).await);
    v0.push(run("buttons_products_ctas_200", async {
        let v3 = v2.get(format!("{}/products", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("href=\"/contact\""), "products missing Get Notified link")?;
        assert_ok(v4.contains("href=\"/services\""), "products missing What We Do link")?;
        let r_contact = v2.get(format!("{}/contact", v1)).send().await.map_err(|e| e.to_string())?;
        let r_svc = v2.get(format!("{}/services", v1)).send().await.map_err(|e| e.to_string())?;
        assert_ok(r_contact.status().is_success(), "products Get Notified must 200")?;
        assert_ok(r_svc.status().is_success(), "products What We Do must 200")?;
        Ok(())
    }).await);
    v0.push(run("buttons_contact_ctas_200", async {
        let v3 = v2.get(format!("{}/contact", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("mailto:") || v4.contains("href=\"/book\""), "contact must have Email or Book link")?;
        assert_ok(v4.contains("href=\"/book\""), "contact must have Book a Call link")?;
        assert_ok(v4.contains("/assets/resume.pdf"), "contact must have resume download")?;
        let r_book = v2.get(format!("{}/book", v1)).send().await.map_err(|e| e.to_string())?;
        let r_resume = v2.get(format!("{}/assets/resume.pdf", v1)).send().await.map_err(|e| e.to_string())?;
        assert_ok(r_book.status().is_success(), "contact Book link must 200")?;
        assert_ok(r_resume.status().is_success(), "contact resume download must 200")?;
        Ok(())
    }).await);
    v0.push(run("buttons_book_nav_200", async {
        let v3 = v2.get(format!("{}/book", v1)).send().await.map_err(|e| e.to_string())?;
        let v4 = v3.text().await.map_err(|e| e.to_string())?;
        assert_ok(v4.contains("booking-prev"), "book must have prev button")?;
        assert_ok(v4.contains("booking-next"), "book must have next button")?;
        assert_ok(v4.contains("mailto:"), "book must have mailto slots")?;
        Ok(())
    }).await);
    v0
}
