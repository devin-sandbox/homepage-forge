# Homepage Forge

A web application for creating and managing web pages, built with Rust (backend) and Qwik (frontend).

## Prerequisites

- Rust (latest stable version)
- Node.js (v18 or later)
- SQLite

## Setup

1. Clone the repository:
```bash
git clone https://github.com/devin-sandbox/homepage-forge.git
cd homepage-forge
```

2. Create a `.env` file:
```bash
echo "DATABASE_URL=sqlite:pages.db" > .env
```

3. Install Rust dependencies and run the backend:
```bash
cargo build
cargo run
```

The backend server will start at `http://localhost:8080`.

4. In a new terminal, install Node.js dependencies:
```bash
npm install
```

5. Run the frontend development server:
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
