# Chunk API Server

API in Rust using Rocket for file chunk management with PostgreSQL database.

## Requirements

- Rust 1.70+
- PostgreSQL 14+
- Cargo

## Database Setup

1. Create a `.env` file in the root directory:
```bash
cp .env.example .env
```

2. Edit `.env` and configure your database URL:
```
DATABASE_URL=postgresql://username:password@localhost/chunk_db
```

3. Create the database and run migrations:
```bash

createdb chunk_db

psql -d chunk_db -f migrations/001_create_chunks_table.sql

# Or use the setup script
./setup_db.sh
```

## How to execute:

```bash
# install lib and run locally
cargo run
```

Server will be available at: `http://localhost:8000`

## Structure

```
poc-chunk-api/
├── src/
│   ├── main.rs          # Main application
│   ├── models.rs        # Database models
│   └── db.rs            # Database operations
├── migrations/
│   └── 001_create_chunks_table.sql
├── data/                # Uploaded chunks storage
├── Cargo.toml
├── .env.example
├── setup_db.sh
└── README.md
```

## Endpoints

### 1. Upload

Upload a file chunk.

**Endpoint:** `POST /upload/<filename>`

**Parameters:**
- `filename` (path) - Name of the file/chunk to be saved

**Body:**
- `multipart/form-data` with `file` field

**Response:**
```json
"Chunk <filename> successfully saved!"
```

**Example with curl:**
```bash
curl -X POST \
  -F "file=@/path/to/file.chunk1" \
  http://localhost:8000/upload/video.mp4.chunk1
```

**Example with JavaScript:**
```javascript
const formData = new FormData();
formData.append('file', fileBlob);

fetch('http://localhost:8000/upload/video.mp4.chunk1', {
  method: 'POST',
  body: formData
})
.then(response => response.json())
.then(data => console.log(data));
```

---

### 2. List Chunks

List all chunks stored on the server with complete metadata.

**Endpoint:** `GET /chunks`

**Response:**
```json
[
  {
    "id": 1,
    "filename": "video.mp4.chunk1",
    "file_path": "data/video.mp4.chunk1",
    "size": 1048576,
    "content_type": "application/octet-stream",
    "created_at": "2024-01-10T10:30:00Z",
    "updated_at": "2024-01-10T10:30:00Z"
  },
  {
    "id": 2,
    "filename": "video.mp4.chunk2",
    "file_path": "data/video.mp4.chunk2",
    "size": 1048576,
    "content_type": "application/octet-stream",
    "created_at": "2024-01-10T10:31:00Z",
    "updated_at": "2024-01-10T10:31:00Z"
  }
]
```

**Example with curl:**
```bash
curl http://localhost:8000/chunks
```

**Example with JavaScript:**
```javascript
fetch('http://localhost:8000/chunks')
  .then(response => response.json())
  .then(chunks => console.log(chunks));
```

---

### 3. Download Specific Chunk

Download a specific chunk by name.

**Endpoint:** `GET /chunk/<filename>`

**Parameters:**
- `filename` (path) - Name of the chunk to download

**Response:**
- Binary file (download)
- Status 404 if file doesn't exist

**Example with curl:**
```bash
curl http://localhost:8000/chunk/video.mp4.chunk1 -o video.mp4.chunk1

```

**Example with JavaScript:**
```javascript
fetch('http://localhost:8000/chunk/video.mp4.chunk1')
  .then(response => response.blob())
  .then(blob => {
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'video.mp4.chunk1';
    a.click();
  });
```

---

### 4. Direct File Access

Static file server for direct access to chunks.

**Endpoint:** `GET /files/<filename>`

**Parameters:**
- `filename` (path) - File name

**Example:**
```bash
curl http://localhost:8000/files/video.mp4.chunk1
```

---

### 5. Delete Chunk

Delete a specific chunk from the server and database.

**Endpoint:** `DELETE /chunk/<filename>`

**Parameters:**
- `filename` (path) - Name of the chunk to delete

**Response:**
```json
"Chunk video.mp4.chunk1 successfully deleted!"
```

**Example with curl:**
```bash
curl -X DELETE http://localhost:8000/chunk/video.mp4.chunk1
```

**Example with JavaScript:**
```javascript
fetch('http://localhost:8000/chunk/video.mp4.chunk1', {
  method: 'DELETE'
})
.then(response => response.json())
.then(data => console.log(data));
```

---

## Complete Usage Flow

### 1. Upload Multiple Chunks

```bash
curl -X POST -F "file=@video.mp4.chunk1" http://localhost:8000/upload/video.mp4.chunk1
curl -X POST -F "file=@video.mp4.chunk2" http://localhost:8000/upload/video.mp4.chunk2
curl -X POST -F "file=@video.mp4.chunk3" http://localhost:8000/upload/video.mp4.chunk3
```

### 2. List Available Chunks

```bash
curl http://localhost:8000/chunks
```

Response:
```json
["video.mp4.chunk1", "video.mp4.chunk2", "video.mp4.chunk3"]
```

### 3. Download and Reconstruct File

```bash
curl http://localhost:8000/chunk/video.mp4.chunk1 -o chunk1
curl http://localhost:8000/chunk/video.mp4.chunk2 -o chunk2
curl http://localhost:8000/chunk/video.mp4.chunk3 -o chunk3

cat chunk1 chunk2 chunk3 > video.mp4

Get-Content chunk1, chunk2, chunk3 -Encoding Byte -ReadCount 0 | Set-Content video.mp4 -Encoding Byte
```

---

## Complete Python Example

```python
import requests
import os

BASE_URL = "http://localhost:8000"

def upload_chunk(filepath, chunk_name):
    with open(filepath, 'rb') as f:
        files = {'file': f}
        response = requests.post(f"{BASE_URL}/upload/{chunk_name}", files=files)
        print(response.json())

def list_chunks():
    response = requests.get(f"{BASE_URL}/chunks")
    return response.json()

def download_chunk(chunk_name, output_path):
    response = requests.get(f"{BASE_URL}/chunk/{chunk_name}")
    with open(output_path, 'wb') as f:
        f.write(response.content)

upload_chunk("video.chunk1", "video.mp4.chunk1")
upload_chunk("video.chunk2", "video.mp4.chunk2")

chunks = list_chunks()
print(f"Available chunks: {chunks}")

download_chunk("video.mp4.chunk1", "downloaded_chunk1")
```

---

## Stack

- [Rocket](https://rocket.rs/) - Framework web para Rust
- [Tokio](https://tokio.rs/) - Runtime assíncrono
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [PostgreSQL](https://www.postgresql.org/) - Database

## Database Schema

```sql
CREATE TABLE chunks (
    id SERIAL PRIMARY KEY,
    filename VARCHAR(255) NOT NULL UNIQUE,
    file_path VARCHAR(500) NOT NULL,
    size BIGINT NOT NULL,
    content_type VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```
