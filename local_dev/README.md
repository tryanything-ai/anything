# Local Development Scripts

This folder contains scripts to easily manage your complete local development environment.

## 🚀 Quick Start

To get **everything** running with one command:

```bash
./local_dev/start.sh
```

This will:
- Start the PostgreSQL database
- Run migrations (if available)
- Seed the database with test data
- Build and start the Rust servers (Main Server + JS Executor)
- Show you what's running and how to access it

## 📋 Available Scripts

### `start.sh` - Start Everything
```bash
./local_dev/start.sh
```
- Starts the database container
- Runs migrations
- Seeds the database with test data
- Builds and starts both Rust servers:
  - **Main Server** (HTTP API on port 3001)
  - **JS Executor** (gRPC on port 50051)
- Handles graceful shutdown with Ctrl+C

### `stop.sh` - Stop Everything
```bash
./local_dev/stop.sh
```
- Stops all Rust servers
- Stops all Docker services
- Cleans up all processes

### `status.sh` - Check Status
```bash
./local_dev/status.sh
```
- Shows what's currently running
- Displays database connection info
- Shows server status and URLs
- Lists available commands

## 🗄️ Database Information

- **Host**: localhost:5432
- **Database**: anything
- **User**: postgres
- **Password**: (check your docker-compose.yml)

## 🖥️ Application Servers

- **Main Server**: http://localhost:3001
- **JS Executor**: localhost:50051 (gRPC)

## 👥 Test Users

The seed script creates these test users:

| Username | Email | Role |
|----------|-------|------|
| user1 | user1@example.com | Owner of Acme Corp |
| user2 | user2@example.com | Owner of Tech Startup |
| user3 | user3@example.com | Member of Acme Corp |
| admin | admin@example.com | Admin of all accounts |

**Note**: Password hashes are placeholders. You'll need to implement proper authentication in your application.

## 🏢 Test Organizations

- **Acme Corporation** (`acme-corp`)
- **Tech Startup Inc** (`tech-startup`) 
- **Freelance Developer** (`freelance-dev`)

## 🛠️ Troubleshooting

### Database won't start
```bash
# Check Docker is running
docker info

# Check logs
docker-compose -f docker-compose.db.yml logs postgres
```

### Can't connect to database
```bash
# Check if container is running
./local_dev/status.sh

# Restart everything
./local_dev/stop.sh
./local_dev/start.sh
```

### Servers won't start
```bash
# Check if ports are in use
lsof -i:3001
lsof -i:50051

# Kill any existing processes
./local_dev/stop.sh
./local_dev/start.sh
```

### Need fresh data
```bash
# Stop everything and restart (includes re-seeding)
./local_dev/stop.sh
./local_dev/start.sh
```

## 📁 File Structure

```
local_dev/
├── start.sh           # Main startup script (does everything!)
├── stop.sh            # Stop everything
├── status.sh          # Check status
└── README.md          # This file
```

## 🔄 Complete Workflow

1. **Start everything**: `./local_dev/start.sh`
2. **Start your web app**: `npm run dev` (or similar)
3. **Check status**: `./local_dev/status.sh`
4. **Stop everything**: `./local_dev/stop.sh` or Ctrl+C

## 🎯 What Gets Started

- ✅ PostgreSQL database
- ✅ Database migrations
- ✅ Test data seeding
- ✅ Main Server (Rust API)
- ✅ JS Executor (Rust gRPC)
- ✅ Environment variables
- ✅ Process management

That's it! Your complete local development environment with one command.
