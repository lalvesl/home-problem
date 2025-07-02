use crate::api::service_counter::get_activities;
use actix_web::{get, HttpResponse};

#[utoipa::path(
        responses(
            (status = 200, description = "Success", body = usize),
            (status = 500, description = "Internal server error", body = str),
        ),
)]
#[get("/activity")]
pub(super) async fn activity() -> HttpResponse {
    let values = get_activities()
        .into_iter()
        .map(|(k, v)| format!("\"{k}\": {v}"))
        .collect::<Vec<String>>()
        .join(",");
    HttpResponse::Ok().body(format!("{{{values}}}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use gutils::anyhow::Result;

    static URI: &str = "/activity";

    #[actix_web::test]
    async fn t_main() -> Result<()> {
        let app = test::init_service(App::new().service(activity)).await;
        let req = test::TestRequest::get().uri(URI).to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        Ok(())
    }
}
