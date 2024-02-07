use std::cmp;
use std::collections::{HashMap};
use actix_multipart::Multipart;
use crate::diff::{differ_by_type, DiffFormData, DiffType};
use crate::embed::get_embed_static_file;
use crate::format::{format_text_by_ext, ExtType, FormatFormData};
use crate::hash::{hash_bytes};
use crate::utils::html_checkbox_to_bool;
use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_lab::__reexports::futures_util::StreamExt;
use minijinja::context;
use minijinja::filters::bool;
use serde::Deserialize;
use uuid::{Uuid};
use crate::consts::SizeUnit;

#[get("/static/{_:.*}")]
async fn statics(path: web::Path<String>) -> impl Responder {
    let content = get_embed_static_file(path.as_str());
    HttpResponse::Ok()
        .content_type(
            mime_guess::from_path(path.as_ref())
                .first_or_octet_stream()
                .as_ref(),
        )
        .body(content)
}

#[get("/hash")]
async fn get_hash(app: web::Data<AppState>) -> impl Responder {
    let result = hash_bytes(b"");
    app.render(
        "hash.twig",
        context! {
            nav_name => "hash",
            result => result,
        },
    )
}

#[allow(dead_code)]
struct MultipartFormField {
    name: String,
    bytes: Vec<u8>,
}

async fn parse_multipart_form(mut form: Multipart) -> HashMap<String, MultipartFormField> {
    let mut map = HashMap::new();

    while let Some(item) = form.next().await {
        let mut field = item.unwrap();
        let mut bufs = Vec::new();
        let name = field.name().to_string();

        while let Some(chunk) = field.next().await {
            let bytes = &chunk.unwrap();
            bufs.append(&mut bytes.to_vec());
        }

        map.insert(name.to_string(), MultipartFormField {
            name,
            bytes: bufs,
        });
    };

    map
}

#[post("/hash")]
async fn post_hash(
    form: Multipart,
    app: web::Data<AppState>,
) -> impl Responder {
    let map = parse_multipart_form(form).await;
    let files = map.get("files").unwrap();
    let content = map.get("content").unwrap();

    if files.bytes.len() > 0 {
        app.render(
            "hash.twig",
            context! {
                nav_name => "hash",
                result => hash_bytes(&files.bytes),
            },
        )
    } else {
        app.render(
            "hash.twig",
            context! {
                nav_name => "hash",
                content => String::from_utf8_lossy(content.bytes.as_slice()).to_string(),
                result => hash_bytes(&content.bytes),
            },
        )
    }
}
#[get("/diff")]
async fn get_diff(app: web::Data<AppState>) -> impl Responder {
    app.render("diff.twig", context! {
        nav_name => "diff",
    })
}
#[post("/diff")]
async fn post_diff(
    form: web::Form<DiffFormData>,
    app: web::Data<AppState>,
) -> impl Responder {
    let unified = &form.unified;

    let result = match unified {
        Some(unified) => {
            if unified.len() > 0 {
                app.diff_to_html(&form.diff_type, unified)
            } else {
                app.diff_to_html(&form.diff_type,
                                 &differ_by_type(DiffType::from_str(&form.diff_type), &form.left, &form.right))
            }
        },
        None => {
            app.diff_to_html(&form.diff_type,
                             &differ_by_type(DiffType::from_str(&form.diff_type), &form.left, &form.right))
        }
    };

    match result {
        Ok(html) => app.render(
            "diff.twig",
            context! {
                nav_name => "diff",
                diff_type => form.diff_type.clone(),
                left => form.left.clone(),
                right => form.right.clone(),
                unified => unified,
                result => html,
            },
        ),
        Err(err) => app.render(
            "diff.twig",
            context! {
                nav_name => "diff",
                diff_type => form.diff_type.clone(),
                left => form.left.clone(),
                right => form.right.clone(),
                error => err,
            },
        ),
    }
}
#[get("/format")]
async fn get_format(app: web::Data<AppState>) -> impl Responder {
    app.render("format.twig", context! {
        nav_name => "format",
    })
}
#[post("/format")]
async fn post_format(form: web::Form<FormatFormData>, app: web::Data<AppState>) -> impl Responder {
    let ext_type = ExtType::from_str(&form.ext);
    let indent2 = html_checkbox_to_bool(&form.indent2);
    let size = form.content.len();

    if size > SizeUnit::MB.to_bytes(2) {
        return app.render(
            "format.twig",
            context! {
                nav_name => "format",
                ext => form.ext.clone(),
                indent2 => indent2,
                content => form.content.clone(),
                error => format!("content size {}, too large > 2MB to format", SizeUnit::MB.from_bytes(size)),
            },
        );
    }

    let result = match ext_type {
        ExtType::JavaScript => app.format_js(&form.content, &format!("{}", indent2).to_string()),
        ExtType::Html => app.format_html(&form.content, &format!("{}", indent2).to_string()),
        ExtType::Css => app.format_css(&form.content, &format!("{}", indent2).to_string()),
        ExtType::TypeScript => {
            if size > SizeUnit::MB.to_bytes(1) {
                // Err(format!("size {}, too large > 1MB to format", SizeUnit::MB.from_bytes(size)))
                app.format_js(&form.content, &format!("{}", indent2).to_string())
            } else {
                format_text_by_ext(ext_type, &form.content, indent2)
            }
        },
        ExtType::Json | ExtType::Markdown | ExtType::Nginx => {
            format_text_by_ext(ext_type, &form.content, indent2)
        }
    };

    match result {
        Ok(formatted) => app.render(
            "format.twig",
            context! {
                nav_name => "format",
                ext => form.ext.clone(),
                indent2 => indent2,
                content => formatted,
            },
        ),
        Err(err) => app.render(
            "format.twig",
            context! {
                nav_name => "format",
                ext => form.ext.clone(),
                indent2 => indent2,
                content => form.content.clone(),
                error => err,
            },
        ),
    }
}

#[derive(Debug, Deserialize)]
struct UuidQuery {
    num: Option<i32>,
}

#[get("/uuid")]
async fn get_uuid(query: web::Query<UuidQuery>, app: web::Data<AppState>) -> impl Responder {
    let mut uuid_map = HashMap::new();
    let num = cmp::max(1, cmp::min(1000, query.num.unwrap_or(5)));
    loop {
        uuid_map.insert(Uuid::new_v4().to_string(), true);
        if uuid_map.len() >= num as usize {
            break;
        }
    }
    let uuids = uuid_map.iter().map(|(k, _)| k).collect::<Vec<_>>();

    app.render("uuid.twig", context! {
        nav_name => "uuid",
        num => num,
        uuids => uuids,
    })
}
