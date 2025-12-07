# vismatch-svc

`vismatch-svc` is a Rust-based microservice for image similarity matching. It allows you to organize images into "projects" and find similar images using perceptual hashing.

## Features

-   **Image Comparison**: Compare an uploaded image against a database of images in a specific project to find the most similar matches.
-   **Image Upload**: Add new images to a project's database.
-   **Perceptual Hashing**: Uses PHash (by default) to identify similar images even if they are resized or slightly modified.
-   **Docker Ready**: Easy deployment with Docker Compose.

## Getting Started

### Prerequisites

-   Docker
-   Docker Compose

### Installation & Running

1.  Clone the repository.
2.  Start the service using Docker Compose:

    ```bash
    docker compose up -d
    ```

    The service will be available at `http://localhost:3000`.

3.  The service mounts the `./image_root` directory. Images are organized by project subdirectories within `image_root`.

    Example structure:
    ```
    image_root/
    ├── my_project/
    │   ├── image1.jpg
    │   ├── image2.png
    │   └── ...
    └── another_project/
        ├── photo.webp
        └── ...
    ```

## API Reference

### 1. Compare Images

Find similar images in a project.

-   **Endpoint**: `POST /diff`
-   **Content-Type**: `application/json`

**Request Body:**

```json
{
  "project_name": "my_project",
  "data": "<base64_encoded_image_string>",
  "with_image": true
}
```

-   `project_name`: Name of the project folder in `image_root`.
-   `data`: Base64 encoded string of the image to compare.
-   `with_image`: If `true`, the response will include the base64 data of the matched images.

**Response:**

```json
{
  "success": true,
  "message": "success",
  "project_name": "my_project",
  "compare_result": [
    {
      "image_name": "image1.jpg",
      "distance": 5.0,
      "data": "<base64_encoded_image_string_or_null>"
    },
    {
      "image_name": "image2.png",
      "distance": 12.0,
      "data": null
    }
  ]
}
```

-   `compare_result`: Returns the top 3 most similar images. Lower `distance` means higher similarity.

### 2. Upload Image

Upload a new image to a project.

-   **Endpoint**: `POST /upload`
-   **Content-Type**: `application/json`

**Request Body:**

```json
{
  "project_name": "my_project",
  "image_name": "new_image.jpg",
  "data": "<base64_encoded_image_string>"
}
```

-   `project_name`: Target project. If the folder doesn't exist, it will be created.
-   `image_name`: Filename to save the image as.
-   `data`: Base64 encoded string of the image.

**Response:**

```json
{
  "success": true,
  "message": "image uploaded and indexed successfully",
  "token": "dummy-deletion-token"
}
```

## Project Structure

-   `src/`: Rust source code.
-   `image_root/`: Directory where images are stored (mounted volume in Docker).
-   `Dockerfile`: Multi-stage Docker build.
-   `compose.yml`: Docker Compose configuration.
