use crate::api::service_counter::activity_add;

use super::entity::Ticker;
use super::service_on_start::DB_TICKERS;
use actix_web::{get, web, HttpResponse};

#[utoipa::path(
        responses(
            (status = 200, description = "Success", body = Vec<Ticker>),
            (status = 404, description = "Not found ticker", body = str),
        ),
)]
#[get("/ticker/{ticker}")]
pub(super) async fn ticker(path: web::Path<String>) -> HttpResponse {
    let search_ticker: String = path.into_inner();
    match DB_TICKERS.get(&search_ticker) {
        Some(ticker) => {
            //Ignore error, because it's impossible
            let _ = activity_add(&search_ticker);
            HttpResponse::Ok().body(serde_json::to_string(&ticker).unwrap())
        }
        None => HttpResponse::NotFound().await.unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use gutils::anyhow::Result;

    static URI: &str = "/ticker/NVDA";

    #[actix_web::test]
    async fn t_activity() -> Result<()> {
        let app = test::init_service(App::new().service(ticker)).await;
        let req = test::TestRequest::get().uri(URI).to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        Ok(())
    }
}
