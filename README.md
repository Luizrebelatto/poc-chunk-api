# Chunk API Server

API in Rust using Rocket for file chunk management.


## How to execute:

```bash
# install lib and run locally
cargo run
```

`http://localhost:8000`

## Structure

```
poc-chunk-api/
├── src/
│   └── main.rs 
├── data/                
├── Cargo.toml          
└── README.md           
```

## Endpoints

### 1. Upload

Faz upload de um chunk de arquivo.

**Endpoint:** `POST /upload/<filename>`

**Parâmetros:**
- `filename` (path) - Nome do arquivo/chunk a ser salvo

**Body:**
- `multipart/form-data` com campo `file`

**Resposta:**
```json
"Chunk <filename> salvo com sucesso!"
```

**Exemplo com curl:**
```bash
curl -X POST \
  -F "file=@/caminho/para/arquivo.chunk1" \
  http://localhost:8000/upload/video.mp4.chunk1
```

**Exemplo com JavaScript:**
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

### 2. Listar Chunks

Lista todos os chunks armazenados no servidor.

**Endpoint:** `GET /chunks`

**Resposta:**
```json
[
  "video.mp4.chunk1",
  "video.mp4.chunk2",
  "video.mp4.chunk3"
]
```

**Exemplo com curl:**
```bash
curl http://localhost:8000/chunks
```

**Exemplo com JavaScript:**
```javascript
fetch('http://localhost:8000/chunks')
  .then(response => response.json())
  .then(chunks => console.log(chunks));
```

---

### 3. Baixar Chunk Específico

Baixa um chunk específico pelo nome.

**Endpoint:** `GET /chunk/<filename>`

**Parâmetros:**
- `filename` (path) - Nome do chunk a ser baixado

**Resposta:**
- Arquivo binário (download)
- Status 404 se o arquivo não existir

**Exemplo com curl:**
```bash
# Baixar e salvar o chunk
curl http://localhost:8000/chunk/video.mp4.chunk1 -o video.mp4.chunk1

# Apenas visualizar informações
curl -I http://localhost:8000/chunk/video.mp4.chunk1
```

**Exemplo com JavaScript:**
```javascript
fetch('http://localhost:8000/chunk/video.mp4.chunk1')
  .then(response => response.blob())
  .then(blob => {
    // Criar URL para download
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'video.mp4.chunk1';
    a.click();
  });
```

---

### 4. Acesso Direto aos Arquivos

Servidor de arquivos estático para acesso direto aos chunks.

**Endpoint:** `GET /files/<filename>`

**Parâmetros:**
- `filename` (path) - Nome do arquivo

**Exemplo:**
```bash
curl http://localhost:8000/files/video.mp4.chunk1
```

---

## Fluxo de Uso Completo

### 1. Upload de Múltiplos Chunks

```bash
# Fazer upload de vários chunks
curl -X POST -F "file=@video.mp4.chunk1" http://localhost:8000/upload/video.mp4.chunk1
curl -X POST -F "file=@video.mp4.chunk2" http://localhost:8000/upload/video.mp4.chunk2
curl -X POST -F "file=@video.mp4.chunk3" http://localhost:8000/upload/video.mp4.chunk3
```

### 2. Listar Chunks Disponíveis

```bash
curl http://localhost:8000/chunks
```

Resposta:
```json
["video.mp4.chunk1", "video.mp4.chunk2", "video.mp4.chunk3"]
```

### 3. Baixar e Reconstruir Arquivo

```bash
# Baixar todos os chunks
curl http://localhost:8000/chunk/video.mp4.chunk1 -o chunk1
curl http://localhost:8000/chunk/video.mp4.chunk2 -o chunk2
curl http://localhost:8000/chunk/video.mp4.chunk3 -o chunk3

# Recombinar os chunks (Linux/Mac)
cat chunk1 chunk2 chunk3 > video.mp4

# Recombinar os chunks (Windows PowerShell)
Get-Content chunk1, chunk2, chunk3 -Encoding Byte -ReadCount 0 | Set-Content video.mp4 -Encoding Byte
```

---

## Exemplo Completo com Python

```python
import requests
import os

BASE_URL = "http://localhost:8000"

# 1. Fazer upload de chunks
def upload_chunk(filepath, chunk_name):
    with open(filepath, 'rb') as f:
        files = {'file': f}
        response = requests.post(f"{BASE_URL}/upload/{chunk_name}", files=files)
        print(response.json())

# 2. Listar chunks
def list_chunks():
    response = requests.get(f"{BASE_URL}/chunks")
    return response.json()

# 3. Baixar chunk
def download_chunk(chunk_name, output_path):
    response = requests.get(f"{BASE_URL}/chunk/{chunk_name}")
    with open(output_path, 'wb') as f:
        f.write(response.content)

# Uso
upload_chunk("video.chunk1", "video.mp4.chunk1")
upload_chunk("video.chunk2", "video.mp4.chunk2")

chunks = list_chunks()
print(f"Chunks disponíveis: {chunks}")

download_chunk("video.mp4.chunk1", "downloaded_chunk1")
```

---

## Códigos de Status HTTP

| Código | Descrição |
|--------|-----------|
| 200    | Requisição bem-sucedida |
| 404    | Chunk não encontrado |
| 500    | Erro interno do servidor |

---

## Desenvolvimento

### Compilar
```bash
cargo build
```

### Rodar testes
```bash
cargo test
```

### Verificar código
```bash
cargo check
```

### Formatar código
```bash
cargo fmt
```

---

## Tecnologias

- [Rocket](https://rocket.rs/) - Framework web para Rust
- [Tokio](https://tokio.rs/) - Runtime assíncrono

---

## Licença

MIT
