# PostgreSQL Setup for Anything

This document describes the PostgreSQL setup for the Anything project, which has been migrated from Supabase to a standard PostgreSQL setup with custom extensions.

## Overview

We've replaced the Supabase PostgreSQL image with a custom PostgreSQL setup that includes:

- **Standard PostgreSQL 15** as the base image
- **pgsodium** extension for encryption (built from source)
- **uuid-ossp** extension for UUID generation
- **Custom auth schema** that mimics Supabase's auth functionality

## Files

- `docker-compose.db.yml` - Docker Compose configuration for PostgreSQL
- `Dockerfile.postgres` - Custom Dockerfile that builds PostgreSQL with pgsodium
- `init-extensions.sql` - Initialization script that sets up extensions and auth schema

## Getting Started

1. **Build and start the database:**
   ```bash
   docker-compose -f docker-compose.db.yml up --build
   ```

2. **The database will be available at:**
   - Host: `localhost`
   - Port: `5432`
   - Database: `anything`
   - Username: `postgres`
   - Password: `postgres`

## Extensions Included

### pgsodium
- Provides encryption functions for secure data storage
- Used by the secrets table for encrypting sensitive data
- Functions available: `crypto_secretbox`, `crypto_secretbox_open`, `randombytes_buf`

### uuid-ossp
- Provides UUID generation functions
- Used throughout the application for generating unique identifiers
- Functions available: `uuid_generate_v4()`

## Auth Schema

We've created a simplified version of Supabase's auth schema that includes:

- `auth.users` - User accounts
- `auth.identities` - OAuth identities
- `auth.sessions` - User sessions
- `auth.uid()` - Function to get current user ID

## Migration from Supabase

The main changes when migrating from Supabase:

1. **No more Supabase-specific features** - We're using standard PostgreSQL
2. **Custom auth implementation** - You'll need to implement your own authentication system
3. **Manual extension management** - Extensions are installed and configured manually
4. **Simplified setup** - No dependency on Supabase's managed services

## Customization

### Adding More Extensions

To add more PostgreSQL extensions, modify the `Dockerfile.postgres`:

```dockerfile
# Install additional extension dependencies
RUN apt-get install -y your-extension-deps

# Build and install the extension
RUN cd /tmp && \
    git clone https://github.com/your-extension && \
    cd your-extension && \
    make && \
    make install
```

Then add the extension to `init-extensions.sql`:

```sql
CREATE EXTENSION IF NOT EXISTS "your-extension";
```

### Modifying Auth Schema

The auth schema in `init-extensions.sql` can be customized to match your authentication requirements. You may need to:

1. Add additional user fields
2. Modify the `auth.uid()` function to work with your auth system
3. Add additional auth-related tables

## Troubleshooting

### Build Issues
If you encounter build issues with pgsodium:

1. Ensure you have sufficient disk space for building
2. Check that all dependencies are properly installed
3. Verify PostgreSQL version compatibility

### Connection Issues
If you can't connect to the database:

1. Check that the container is running: `docker ps`
2. Verify the port mapping: `docker port anything-postgres`
3. Check container logs: `docker logs anything-postgres`

### Extension Issues
If extensions aren't loading:

1. Check the initialization logs in the container
2. Verify the extension files are properly installed
3. Ensure the extension is enabled in `init-extensions.sql`

## Security Considerations

1. **Default credentials** - Change the default postgres password in production
2. **Network access** - Restrict database access to your application only
3. **Encryption keys** - Ensure pgsodium keys are properly managed
4. **Auth implementation** - Implement proper authentication and authorization

## Production Deployment

For production deployment:

1. Use environment variables for sensitive configuration
2. Implement proper backup strategies
3. Set up monitoring and logging
4. Configure SSL/TLS for database connections
5. Use a managed PostgreSQL service if possible
