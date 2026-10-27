use apple_cf::cf::{CFURLComponent, CFURL};

fn main() {
    let url = CFURL::parse("https://example.com:8443/a%20b/index.html?x=1#/route", None).unwrap();
    assert_eq!(
        url.component(CFURLComponent::Scheme).as_deref(),
        Some("https")
    );
    assert_eq!(
        url.component(CFURLComponent::Authority).as_deref(),
        Some("example.com:8443")
    );
    assert_eq!(
        url.component(CFURLComponent::Host).as_deref(),
        Some("example.com")
    );
    assert_eq!(url.component(CFURLComponent::Port).as_deref(), Some("8443"));
    assert_eq!(
        url.component(CFURLComponent::Path).as_deref(),
        Some("/a%20b/index.html")
    );
    assert_eq!(url.component(CFURLComponent::Query).as_deref(), Some("x=1"));
    assert_eq!(
        url.component(CFURLComponent::Fragment).as_deref(),
        Some("/route")
    );
    let joined = CFURL::parse("../page.html#next", Some(&url)).unwrap();
    assert_eq!(
        joined.absolute_string().to_string(),
        "https://example.com:8443/page.html#next"
    );
    let file = CFURL::parse("file:///Library/Example/A%20B/%23page.html", None).unwrap();
    assert_eq!(
        file.file_system_path().unwrap().to_string(),
        "/Library/Example/A B/#page.html"
    );
    println!("CFURL components, relative resolution and file conversion passed");
}
