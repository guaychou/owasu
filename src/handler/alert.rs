use {
    crate::{
        domain::seatalk::Seatalk, error::AppError, extractor::JsonExtractor,
        handler::response::Response, model::alertmanager::AlertManagerRequestBody,
    },
    axum::{extract::Extension, Json},
    tracing::instrument,
};

#[instrument(name = "alert_handler" skip(seatalk))]
pub async fn alert(
    JsonExtractor(req): JsonExtractor<AlertManagerRequestBody>,
    Extension(seatalk): Extension<Seatalk>,
) -> Result<Json<Response>, AppError> {
    let data = seatalk.send_alert(req).await?;
    if data.error_check() {
        return Err(AppError::SeatalkError(data));
    }
    Ok(Response::default().into())
}
