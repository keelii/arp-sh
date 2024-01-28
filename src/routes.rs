use crate::diff::{differ_by_type, DiffFormData, DiffType};
use crate::embed::get_embed_static_file;
use crate::format::{format_text_by_ext, ExtType, FormatFormData};
use crate::hash::{hash_bytes, HashFormData};
use crate::utils::html_checkbox_to_bool;
use crate::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use minijinja::context;

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
#[post("/hash")]
async fn post_hash(
    form: web::Form<HashFormData>,
    app: web::Data<AppState>,
) -> impl Responder {
    let result = hash_bytes(form.content.as_bytes());
    app.render(
        "hash.twig",
        context! {
            nav_name => "hash",
            content => form.content.clone(),
            result => result
        },
    )
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

    let result = match ext_type {
        ExtType::JavaScript => app.format_js(&form.content, &format!("{}", indent2).to_string()),
        ExtType::Html => app.format_html(&form.content, &format!("{}", indent2).to_string()),
        ExtType::Css => app.format_css(&form.content, &format!("{}", indent2).to_string()),
        ExtType::TypeScript | ExtType::Json | ExtType::Markdown | ExtType::Nginx => {
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
