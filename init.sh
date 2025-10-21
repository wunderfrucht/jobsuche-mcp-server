#!/bin/bash

# ============================================================================
# MCP Server Template Initialization Script
# ============================================================================
# This script automates the setup process for the MCP server template by:
# - Prompting for project details (package name, author, etc.)
# - Validating user inputs
# - Replacing all placeholders across the project
# - Renaming the template directory to the new package name
# - Providing a comprehensive summary of changes
# ============================================================================

set -e # Exit on error

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ============================================================================
# Helper Functions
# ============================================================================

print_header() {
  echo ""
  echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo -e "${BLUE}  $1${NC}"
  echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo ""
}

print_success() {
  echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
  echo -e "${RED}✗ $1${NC}"
}

print_warning() {
  echo -e "${YELLOW}⚠ $1${NC}"
}

print_info() {
  echo -e "${BLUE}ℹ $1${NC}"
}

# Validate package name (npm compatible)
validate_package_name() {
  local name="$1"

  # Check if empty
  if [[ -z "$name" ]]; then
    print_error "Package name cannot be empty"
    return 1
  fi

  # Check length (214 chars max for npm)
  if [[ ${#name} -gt 214 ]]; then
    print_error "Package name must be 214 characters or less"
    return 1
  fi

  # Check for valid characters (lowercase letters, numbers, hyphens, underscores)
  if [[ ! "$name" =~ ^[a-z0-9_-]+$ ]]; then
    print_error "Package name must contain only lowercase letters, numbers, hyphens, and underscores"
    return 1
  fi

  # Cannot start with dot or underscore
  if [[ "$name" =~ ^[._] ]]; then
    print_error "Package name cannot start with a dot or underscore"
    return 1
  fi

  return 0
}

# Validate npm scope
validate_npm_scope() {
  local scope="$1"

  # Check if starts with @
  if [[ ! "$scope" =~ ^@ ]]; then
    print_error "NPM scope must start with @"
    return 1
  fi

  # Check for valid characters after @
  local scope_name="${scope:1}"
  if [[ ! "$scope_name" =~ ^[a-z0-9_-]+$ ]]; then
    print_error "NPM scope must contain only lowercase letters, numbers, hyphens, and underscores after @"
    return 1
  fi

  return 0
}

# Validate email address
validate_email() {
  local email="$1"

  # Basic email validation regex
  if [[ ! "$email" =~ ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$ ]]; then
    print_error "Invalid email format"
    return 1
  fi

  return 0
}

# Convert package name to title case for descriptions
to_title_case() {
  local name="$1"
  # Replace hyphens and underscores with spaces, capitalize each word
  echo "$name" | sed 's/[-_]/ /g' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) tolower(substr($i,2))}1'
}

# Check if already initialized
check_if_initialized() {
  # Check if template-mcp-server directory still exists
  if [[ ! -d "template-mcp-server" ]]; then
    print_warning "The 'template-mcp-server' directory does not exist."
    print_warning "This template may have already been initialized."
    echo ""
    read -r -p "Do you want to continue anyway? (y/n): " -n 1
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
      print_info "Initialization cancelled."
      exit 0
    fi
  fi

  # Check if placeholders exist
  if ! grep -r "@yourusername" . --exclude-dir={.git,target,node_modules,.claude} >/dev/null 2>&1; then
    print_warning "No '@yourusername' placeholders found."
    print_warning "This template may have already been initialized."
    echo ""
    read -r -p "Do you want to continue anyway? (y/n): " -n 1
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
      print_info "Initialization cancelled."
      exit 0
    fi
  fi
}

# Create backup
create_backup() {
  local backup_dir
  backup_dir="backup-$(date +%Y%m%d-%H%M%S)"
  print_info "Creating backup at: $backup_dir"

  # Create backup excluding unnecessary directories
  mkdir -p "$backup_dir"
  rsync -a --exclude='.git' --exclude='target' --exclude='node_modules' --exclude='.claude' --exclude="$backup_dir" . "$backup_dir/"

  print_success "Backup created successfully"
  echo "$backup_dir"
}

# ============================================================================
# Main Script
# ============================================================================

print_header "MCP Server Template Initialization"

echo "This script will help you customize your MCP server template."
echo "It will replace placeholder values throughout the project."
echo ""

# Check if already initialized
check_if_initialized

echo ""
read -r -p "Do you want to create a backup before proceeding? (recommended, y/n): " -n 1
echo ""
BACKUP_DIR=""
if [[ $REPLY =~ ^[Yy]$ ]]; then
  BACKUP_DIR=$(create_backup)
fi

# ============================================================================
# Collect User Input
# ============================================================================

print_header "Project Configuration"

# Package name
while true; do
  echo -e "${BLUE}Package name${NC} (lowercase, hyphens ok)"
  read -r -p "Enter package name [my-mcp-server]: " PACKAGE_NAME
  PACKAGE_NAME=${PACKAGE_NAME:-my-mcp-server}

  if validate_package_name "$PACKAGE_NAME"; then
    print_success "Package name: $PACKAGE_NAME"
    break
  fi
done
echo ""

# NPM scope
while true; do
  echo -e "${BLUE}NPM scope${NC} (must start with @)"
  read -r -p "Enter NPM scope [@username]: " NPM_SCOPE
  NPM_SCOPE=${NPM_SCOPE:-@username}

  if validate_npm_scope "$NPM_SCOPE"; then
    print_success "NPM scope: $NPM_SCOPE"
    break
  fi
done
echo ""

# Repository owner (without @)
echo -e "${BLUE}Repository owner${NC} (GitHub username, without @)"
read -r -p "Enter repository owner [yourusername]: " REPO_OWNER
REPO_OWNER=${REPO_OWNER:-yourusername}
print_success "Repository owner: $REPO_OWNER"
echo ""

# Author name
echo -e "${BLUE}Author name${NC}"
read -r -p "Enter author name [Your Name]: " AUTHOR_NAME
AUTHOR_NAME=${AUTHOR_NAME:-Your Name}
print_success "Author name: $AUTHOR_NAME"
echo ""

# Author email
while true; do
  echo -e "${BLUE}Author email${NC}"
  read -r -p "Enter author email [your.email@example.com]: " AUTHOR_EMAIL
  AUTHOR_EMAIL=${AUTHOR_EMAIL:-your.email@example.com}

  if validate_email "$AUTHOR_EMAIL"; then
    print_success "Author email: $AUTHOR_EMAIL"
    break
  fi
done
echo ""

# Description
echo -e "${BLUE}Package description${NC}"
read -r -p "Enter description [An MCP server]: " DESCRIPTION
DESCRIPTION=${DESCRIPTION:-An MCP server}
print_success "Description: $DESCRIPTION"
echo ""

# Derive additional values
PACKAGE_TITLE=$(to_title_case "$PACKAGE_NAME")
AUTHOR_INFO="$AUTHOR_NAME <$AUTHOR_EMAIL>"

# ============================================================================
# Confirmation
# ============================================================================

print_header "Configuration Summary"

echo -e "${BLUE}Package name:${NC}      $PACKAGE_NAME"
echo -e "${BLUE}Package title:${NC}     $PACKAGE_TITLE"
echo -e "${BLUE}NPM scope:${NC}         $NPM_SCOPE"
echo -e "${BLUE}Repository owner:${NC}  $REPO_OWNER"
echo -e "${BLUE}Author:${NC}            $AUTHOR_INFO"
echo -e "${BLUE}Description:${NC}       $DESCRIPTION"
echo ""

read -r -p "Proceed with initialization? (y/n): " -n 1
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
  print_info "Initialization cancelled."
  if [[ -n "$BACKUP_DIR" ]]; then
    print_info "Backup preserved at: $BACKUP_DIR"
  fi
  exit 0
fi

# ============================================================================
# Perform Replacements
# ============================================================================

print_header "Replacing Placeholders"

# Track changes
declare -a CHANGED_FILES=()

# Function to replace in a file
replace_in_file() {
  local file="$1"
  local search="$2"
  local replace="$3"
  local description="$4"

  if [[ -f "$file" ]] && grep -q "$search" "$file" 2>/dev/null; then
    if [[ "$OSTYPE" == "darwin"* ]]; then
      # macOS
      sed -i '' "s|$search|$replace|g" "$file"
    else
      # Linux
      sed -i "s|$search|$replace|g" "$file"
    fi

    # Add to changed files if not already there
    local found=0
    for changed_file in "${CHANGED_FILES[@]}"; do
      if [[ "$changed_file" == "$file" ]]; then
        found=1
        break
      fi
    done
    if [[ $found -eq 0 ]]; then
      CHANGED_FILES+=("$file")
    fi

    return 0
  fi
  return 1
}

# Function to replace in all files
replace_globally() {
  local search="$1"
  local replace="$2"
  local description="$3"

  print_info "Replacing: $description"

  local count=0
  while IFS= read -r -d '' file; do
    if replace_in_file "$file" "$search" "$replace" "$description"; then
      ((count++))
    fi
  done < <(find . -type f \
    -not -path '*/\.*' \
    -not -path '*/target/*' \
    -not -path '*/node_modules/*' \
    -not -path '*/backup-*/*' \
    -not -name 'init.sh' \
    -print0)

  if [[ $count -gt 0 ]]; then
    print_success "Updated $count files"
  else
    print_warning "No files needed updating"
  fi
}

# Perform replacements in order of specificity
echo ""

# 1. Replace @yourusername/template-mcp-server (full scoped package name)
replace_globally "@yourusername/template-mcp-server" "$NPM_SCOPE/$PACKAGE_NAME" "@yourusername/template-mcp-server → $NPM_SCOPE/$PACKAGE_NAME"

# 2. Replace template-mcp-server (package name in various contexts)
replace_globally "template-mcp-server" "$PACKAGE_NAME" "template-mcp-server → $PACKAGE_NAME"

# 3. Replace template_mcp_server (Rust library name)
RUST_LIB_NAME=$(echo "$PACKAGE_NAME" | tr '-' '_')
replace_globally "template_mcp_server" "$RUST_LIB_NAME" "template_mcp_server → $RUST_LIB_NAME"

# 4. Replace Template MCP Server (title)
replace_globally "Template MCP Server" "$PACKAGE_TITLE" "Template MCP Server → $PACKAGE_TITLE"

# 5. Replace @yourusername (scope only)
replace_globally "@yourusername" "$NPM_SCOPE" "@yourusername → $NPM_SCOPE"

# 6. Replace yourusername in GitHub URLs
replace_globally "yourusername/template-mcp-server" "$REPO_OWNER/$PACKAGE_NAME" "GitHub repo URLs"
replace_globally "yourusername/your-mcp-server" "$REPO_OWNER/$PACKAGE_NAME" "GitHub repo URLs (examples)"

# 7. Replace author info
replace_globally "Your Name <your.email@example.com>" "$AUTHOR_INFO" "Author information"

# 8. Replace description if not default
if [[ "$DESCRIPTION" != "An MCP server" ]]; then
  replace_globally "A template MCP server using PulseEngine MCP framework" "$DESCRIPTION" "Package description"
  replace_globally "Template MCP Server - A Model Context Protocol server template using PulseEngine MCP framework" "$DESCRIPTION" "Package description (npm)"
fi

# ============================================================================
# Rename Directory
# ============================================================================

if [[ -d "template-mcp-server" ]] && [[ "$PACKAGE_NAME" != "template-mcp-server" ]]; then
  print_header "Renaming Directory"

  print_info "Renaming: template-mcp-server → $PACKAGE_NAME"

  if [[ -d "$PACKAGE_NAME" ]]; then
    print_error "Directory '$PACKAGE_NAME' already exists!"
    print_error "Please remove it first or choose a different package name."
    exit 1
  fi

  mv "template-mcp-server" "$PACKAGE_NAME"
  print_success "Directory renamed successfully"

  # Update workspace member in Cargo.toml
  if [[ -f "Cargo.toml" ]]; then
    if [[ "$OSTYPE" == "darwin"* ]]; then
      sed -i '' "s|\"template-mcp-server\"|\"$PACKAGE_NAME\"|g" "Cargo.toml"
    else
      sed -i "s|\"template-mcp-server\"|\"$PACKAGE_NAME\"|g" "Cargo.toml"
    fi
    print_success "Updated Cargo.toml workspace member"
  fi
fi

# ============================================================================
# Validation
# ============================================================================

print_header "Validating Changes"

# Check for remaining placeholders
REMAINING_YOURUSERNAME=$(grep -r "@yourusername" . \
  --exclude-dir={.git,target,node_modules,.claude,backup-*} \
  --exclude="init.sh" 2>/dev/null | wc -l | tr -d ' ')

REMAINING_TEMPLATE=$(grep -r "template-mcp-server" . \
  --exclude-dir={.git,target,node_modules,.claude,backup-*} \
  --exclude="init.sh" \
  --exclude="README.md" \
  --exclude="PUBLISHING.md" 2>/dev/null | wc -l | tr -d ' ')

if [[ "$REMAINING_YOURUSERNAME" -eq 0 ]]; then
  print_success "No '@yourusername' placeholders remaining"
else
  print_warning "Found $REMAINING_YOURUSERNAME remaining '@yourusername' references"
  print_info "These may be in documentation examples (check manually)"
fi

if [[ "$REMAINING_TEMPLATE" -eq 0 ]]; then
  print_success "No 'template-mcp-server' placeholders remaining (except docs)"
else
  print_info "Found $REMAINING_TEMPLATE remaining 'template-mcp-server' references"
  print_info "These may be in documentation examples (check manually)"
fi

# ============================================================================
# Summary
# ============================================================================

print_header "Initialization Complete!"

echo -e "${GREEN}Your MCP server has been successfully initialized!${NC}"
echo ""
echo -e "${BLUE}Files modified:${NC} ${#CHANGED_FILES[@]}"
if [[ -n "$BACKUP_DIR" ]]; then
  echo -e "${BLUE}Backup location:${NC} $BACKUP_DIR"
fi
echo ""

print_header "Next Steps"

echo "1. Review the changes:"
echo "   git status"
echo "   git diff"
echo ""

echo "2. Build your server:"
echo "   cargo build"
echo ""

echo "3. Test your server:"
echo "   cargo run"
echo ""

echo "4. Implement your custom tools in:"
echo "   $PACKAGE_NAME/src/lib.rs"
echo ""

echo "5. Update the README with your project details:"
echo "   README.md"
echo ""

echo "6. When ready, commit your changes:"
echo "   git add ."
echo "   git commit -m 'feat: initialize $PACKAGE_NAME from template'"
echo ""

print_header "Publishing"

echo "To publish your server to npm:"
echo ""
echo "1. Build release binaries:"
echo "   ./scripts/build-all.sh"
echo ""
echo "2. Publish to npm:"
echo "   cd npm && npm publish"
echo ""

echo "For detailed publishing instructions, see:"
echo "   PUBLISHING.md"
echo ""

print_success "Happy coding! 🚀"
echo ""
