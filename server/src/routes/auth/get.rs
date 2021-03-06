use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Responder, Result};

use crate::app_state::AppState;
use crate::error::NASError;
use crate::templates::AuthPageParams;
use crate::CONFIG;

pub async fn get(identity: Identity, app_state: web::Data<AppState>) -> Result<impl Responder> {
    let identity = identity.identity();
    let templates = &app_state.templates;

    if identity.is_none() {
        let response_body = templates
            .render(
                "auth",
                &AuthPageParams {
                    theme: CONFIG.theme.clone(),
                    message: None,
                    logged_in: false,
                    redirect_url: None,
                },
            )
            .map_err(|e| NASError::TemplateRenderError {
                template: "auth".to_string(),
                error: e.to_string(),
            })?;

        return Ok(HttpResponse::Ok()
            .header(http::header::CONTENT_TYPE, "text/html;charset=utf-8")
            .body(response_body));
    }

    let response_body = templates
        .render(
            "auth",
            &AuthPageParams {
                theme: CONFIG.theme.clone(),
                message: None,
                logged_in: true,
                redirect_url: Some("/fs/".to_string()),
            },
        )
        .map_err(|e| NASError::TemplateRenderError {
            template: "auth".to_string(),
            error: e.to_string(),
        })?;

    Ok(HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "text/html;charset=utf-8")
        .body(response_body))
}
