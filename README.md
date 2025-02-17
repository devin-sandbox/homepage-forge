# Homepage Forge

A web application for creating and managing web pages, built with Rust (backend) and Qwik (frontend).

## Prerequisites

- Rust (latest stable version)
- Node.js (v18 or later)
- SQLite

## Setup

### Backend Setup

1. Navigate to the backend directory:
```bash
cd backend
```

2. Create a `.env` file:
```bash
echo "DATABASE_URL=sqlite:pages.db" > .env
```

3. Install dependencies and run:
```bash
cargo build
cargo run
```

The backend server will start at `http://localhost:8080`.

### Frontend Setup

1. Navigate to the frontend directory:
```bash
cd frontend
```

2. Install dependencies:
```bash
npm install
```

3. Run the development server:
```bash
npm run dev
```

The frontend will be available at `http://localhost:5173`.

## API Endpoints

- `GET /api/pages` - Get all pages
- `POST /api/pages` - Create a new page
- `GET /api/pages/{id}` - Get a specific page
- `PUT /api/pages/{id}` - Update a page
- `DELETE /api/pages/{id}` - Delete a page

## Features

- Create, read, update, and delete web pages
- Markdown content editing
- SQLite database for persistent storage
- Fast and efficient Rust backend
- Modern Qwik frontend with optimized performance
