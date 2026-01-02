use axum::{
    extract::Path,
    http::{StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::fs;
use tower_http::{
    compression::CompressionLayer, 
    services::ServeDir,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handle_home))
        .route("/blog/:id", get(handle_blog_detail))
        
        .nest_service("/assets", ServeDir::new("public/assets"))
        .fallback_service(ServeDir::new("public"))
        
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010")
        .await
        .unwrap();
    println!("📍 http://localhost:3010");
    axum::serve(listener, app).await.unwrap();
}

async fn handle_home() -> Response {
    let meta = MetaTags {
        title: "Edgar Manuel Janota",
        description: "Fullstack Developer especializado em TypeScript, Node.js, Clean Architecture, Microservices e Event-Driven Systems. ",
        url: "https://ndulo.pages.dev",
        og_image: "https://ndulo.pages.dev/og-home.jpg",
        keywords: "Edgar Janota, Fullstack Developer Angola, TypeScript Developer, Rust Developer, Clean Architecture, Microservices, Event-Driven Architecture, DDD, CQRS, SAGA Pattern, Node.js, React, PostgreSQL, Docker, MODRESS, InstantPay",
        author: "Edgar Manuel Janota",
        twitter_handle: "@eddiendulo",
        structured_data: Some(get_person_schema()),
    };
    
    inject_meta_tags(meta).await
}

async fn handle_blog_detail(Path(id): Path<String>) -> Response {
    let (title, description, keywords) = match id.as_str() {
        "clean-architecture-typescript" => (
            "Clean Architecture em TypeScript | Edgar Janota",
            "Guia de implementação de Clean Architecture em projetos Node.js e TypeScript, com exemplos práticos do projeto MODRESS e InstantPay.",
            "Clean Architecture, TypeScript, Node.js, DDD, Dependency Injection, Result Pattern, Functional Programming"
        ),
        "rust-vs-nodejs-performance" => (
            "Rust vs Node.js: Performance em Produção | Edgar Janota",
            "Análise técnica comparando Rust e Node.js em cenários reais. Benchmarks, trade-offs e quando usar cada tecnologia baseado em projetos reais.",
            "Rust, Node.js, Performance, Benchmarks, Backend Development, Systems Programming"
        ),
        "event-driven-microservices" => (
            "Arquitetura Event-Driven com SAGA Pattern | Edgar Janota",
            "Como implementei microservices event-driven com SAGA pattern no InstantPay. RabbitMQ, Redis, circuit breakers e eventual consistency.",
            "Microservices, Event-Driven Architecture, SAGA Pattern, RabbitMQ, Redis, Distributed Systems"
        ),
        "modress-case-study" => (
            "Case Study: MODRESS - Marketplace Angolano | Edgar Janota",
            "Como construímos o MODRESS, marketplace de moda que venceu a competição BNA. Stack técnica, desafios e soluções de arquitetura.",
            "MODRESS, Marketplace, TypeScript, React, PostgreSQL, Event-Driven, Angola Tech"
        ),
        _ => (
            "Blog | Edgar Manuel Janota",
            "Artigos técnicos sobre desenvolvimento backend, arquitetura de software, performance e tecnologias modernas.",
            "Software Engineering, Backend Development, Architecture, Performance"
        ),
    };
    
    let meta = MetaTags {
        title,
        description,
        url: &format!("https://ndulo.pages.dev/blog/{}", id),
        og_image: &format!("https://ndulo.pages.dev/og-blog-{}.jpg", id),
        keywords,
        author: "Edgar Manuel Janota",
        twitter_handle: "@edgarjanota",
        structured_data: Some(get_article_schema(&id, title, description)),
    };
    
    inject_meta_tags(meta).await
}

struct MetaTags<'a> {
    title: &'a str,
    description: &'a str,
    url: &'a str,
    og_image: &'a str,
    keywords: &'a str,
    author: &'a str,
    twitter_handle: &'a str,
    structured_data: Option<String>,
}

async fn inject_meta_tags(meta: MetaTags<'_>) -> Response {
    let html_template = match fs::read_to_string("public/index.html") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Erro ao ler public/index.html: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load HTML").into_response()
        }
    };

    let structured_data_tag = meta.structured_data
        .map(|data| format!(r#"<script type="application/ld+json">{}</script>"#, data))
        .unwrap_or_default();

    let meta_tags = format!(
        r#"<title>{title}</title>
    <meta name="description" content="{description}">
    <meta name="keywords" content="{keywords}">
    <meta name="author" content="{author}">
    <link rel="canonical" href="{url}">
    
    <!-- Open Graph / Facebook -->
    <meta property="og:type" content="website">
    <meta property="og:url" content="{url}">
    <meta property="og:title" content="{title}">
    <meta property="og:description" content="{description}">
    <meta property="og:image" content="{og_image}">
    <meta property="og:image:width" content="1200">
    <meta property="og:image:height" content="630">
    <meta property="og:locale" content="pt_AO">
    <meta property="og:locale:alternate" content="en_US">
    <meta property="og:site_name" content="Edgar Janota Portfolio">
    
    <!-- Twitter -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:site" content="{twitter_handle}">
    <meta name="twitter:creator" content="{twitter_handle}">
    <meta name="twitter:url" content="{url}">
    <meta name="twitter:title" content="{title}">
    <meta name="twitter:description" content="{description}">
    <meta name="twitter:image" content="{og_image}">
    
    <meta name="robots" content="index, follow, max-snippet:-1, max-image-preview:large, max-video-preview:-1">
    <meta name="googlebot" content="index, follow">
    <meta name="google" content="notranslate">
    <meta http-equiv="content-language" content="pt-AO">
    
    <!-- PWA / Mobile -->
    <meta name="mobile-web-app-capable" content="yes">
    <meta name="apple-mobile-web-app-capable" content="yes">
    <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent">
    <meta name="theme-color" content='#000000'>
    
    <!-- Geo Tags -->
    <meta name="geo.region" content="AO-LUA">
    <meta name="geo.placename" content="Luanda">
    <meta name="geo.position" content="-8.8383;13.2344">
    <meta name="ICBM" content="-8.8383, 13.2344">
    
    {structured_data}
    "#,
        title = meta.title,
        description = meta.description,
        url = meta.url,
        og_image = meta.og_image,
        keywords = meta.keywords,
        author = meta.author,
        twitter_handle = meta.twitter_handle,
        structured_data = structured_data_tag,
    );

    let final_html = html_template.replace("</head>", &format!("{}\n</head>", meta_tags));

    Html(final_html).into_response()
}

fn get_person_schema() -> String {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "Person",
        "name": "Edgar Manuel Janota",
        "alternateName": "Ndulo",
        "jobTitle": "Fullstack Developer & DevOps Engineer",
        "email": "eddiendulo@gmail.com",
        "telephone": "+244925885405",
        "url": "https://ndulo.pages.dev",
        "image": "https://ndulo.pages.dev/avatar.jpg",
        "address": {
            "@type": "PostalAddress",
            "addressLocality": "Luanda",
            "addressCountry": "AO"
        },
        "sameAs": [
            "https://github.com/ndulomk",
            "https://linkedin.com/in/edgar-manuel-janota-387329328"
        ],
        "knowsAbout": [
            "TypeScript",
            "Rust",
            "Node.js",
            "React",
            "Clean Architecture",
            "Microservices",
            "Event-Driven Architecture",
            "Domain-Driven Design",
            "PostgreSQL",
            "Docker",
            "CI/CD"
        ],
        "alumniOf": "Software Engineering",
        "worksFor": {
            "@type": "Organization",
            "name": "MODRESS",
            "url": "https://modress.shop"
        }
    }).to_string()
}

fn get_article_schema(id: &str, title: &str, description: &str) -> String {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "TechArticle",
        "headline": title,
        "description": description,
        "url": format!("https://ndulo.pages.dev/blog/{}", id),
        "datePublished": "2024-01-01T00:00:00Z",
        "dateModified": "2024-01-01T00:00:00Z",
        "author": {
            "@type": "Person",
            "name": "Edgar Manuel Janota",
            "url": "https://ndulo.pages.dev"
        },
        "publisher": {
            "@type": "Person",
            "name": "Edgar Manuel Janota"
        },
        "image": format!("https://ndulo.pages.dev/og-blog-{}.jpg", id),
        "articleSection": "Software Engineering",
        "inLanguage": "pt-AO",
        "keywords": "software architecture, backend development, typescript, rust"
    }).to_string()
}