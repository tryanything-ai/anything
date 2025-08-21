#!/bin/bash

# Test script for the new authentication system
BASE_URL="http://localhost:3001"

echo "🚀 Testing Custom Authentication System"
echo "======================================="

# Test registration
echo "📝 Testing user registration..."
REGISTER_RESPONSE=$(curl -s -X POST "$BASE_URL/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "testpassword123"
  }')

echo "Register Response: $REGISTER_RESPONSE"

# Extract token from registration response
TOKEN=$(echo $REGISTER_RESPONSE | grep -o '"token":"[^"]*"' | sed 's/"token":"\([^"]*\)"/\1/')

if [ -n "$TOKEN" ]; then
    echo "✅ Registration successful, token received"
    
    # Test getting user info
    echo "👤 Testing /auth/me endpoint..."
    ME_RESPONSE=$(curl -s -X GET "$BASE_URL/auth/me" \
      -H "Authorization: Bearer $TOKEN")
    echo "Me Response: $ME_RESPONSE"
    
    # Test creating a secret (replace account_id with a real one)
    echo "🔐 Testing secret creation..."
    SECRET_RESPONSE=$(curl -s -X POST "$BASE_URL/account/00000000-0000-0000-0000-000000000000/secret_new" \
      -H "Authorization: Bearer $TOKEN" \
      -H "Content-Type: application/json" \
      -d '{
        "secret_name": "test_api_key",
        "secret_value": "sk-1234567890abcdef",
        "description": "Test API key for demonstration"
      }')
    echo "Secret Response: $SECRET_RESPONSE"
    
    # Test logout
    echo "🚪 Testing logout..."
    LOGOUT_RESPONSE=$(curl -s -X POST "$BASE_URL/auth/logout" \
      -H "Authorization: Bearer $TOKEN")
    echo "Logout Response: $LOGOUT_RESPONSE"
    
else
    echo "❌ Registration failed"
fi

# Test login
echo "🔑 Testing user login..."
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "testpassword123"
  }')

echo "Login Response: $LOGIN_RESPONSE"

echo ""
echo "✨ Testing complete!"
echo "Note: Make sure the server is running and the database migration has been applied."
echo "To run the migration: psql \$DATABASE_URL -f migrations/001_setup_pgsodium_and_auth.sql"
