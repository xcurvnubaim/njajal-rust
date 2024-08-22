use axum::{body::Bytes, extract::Multipart, http::StatusCode, BoxError, Json};
use tokio_util::io::StreamReader;
use futures::{Stream, TryStreamExt};
use std::io;
use tokio::{fs::File, io::BufWriter};

const UPLOADS_DIRECTORY: &str = "uploads";

use crate::{
    app::response::{error::ErrorResponse, success::SuccessResponse},
    presentation::dto::upload_dto::UploadFileDTO,
};

pub async fn upload(
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<SuccessResponse<UploadFileDTO>>), (StatusCode, Json<ErrorResponse>)> {
    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        match stream_to_file(&file_name, field).await {
            Ok(_) => {
                return Ok((
                    StatusCode::OK,
                    Json(SuccessResponse::new(
                        "Successfully uploaded file".to_string(),
                        UploadFileDTO {
                            file_name,
                        },
                    )),
                ))
            }
            Err((status_code, message)) => {
                return Err((
                    status_code,
                    Json(ErrorResponse::new("Failed to upload file".to_string(), message)),
                ));
            }
        }
    }
    Ok((
        StatusCode::OK,
        Json(SuccessResponse::new(
            "Successfully uploaded file".to_string(),
            UploadFileDTO {
                file_name: "file_name".to_string(),
            },
        )),
    ))
}


// Save a `Stream` to a file
async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<(), (StatusCode, String)>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }

    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        // Create the file. `File` implements `AsyncWrite`.
        let path = std::path::Path::new(UPLOADS_DIRECTORY).join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

// to prevent directory traversal attacks we ensure the path consists of exactly one normal
// component
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}