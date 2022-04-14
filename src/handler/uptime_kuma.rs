use {
    crate::{
        domain::seatalk::Seatalk, error::AppError, extractor::JsonExtractor,
        handler::response::Response, model::uptime_kuma::UptimeKumaRequestBody,
    },
    axum::{extract::Extension, Json},
    tracing::instrument,
};

#[instrument(name = "kuma_handler" skip(seatalk))]
pub async fn kuma_alert(
    JsonExtractor(req): JsonExtractor<UptimeKumaRequestBody>,
    Extension(seatalk): Extension<Seatalk>,
) -> Result<Json<Response>, AppError> {
    let data = seatalk.send_alert(req).await?;
    if data.error_check() {
        return Err(AppError::SeatalkError(data));
    }
    Ok(Response::default().into())
}
