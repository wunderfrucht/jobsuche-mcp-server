#!/bin/bash
# Validation script to check for unreplaced template placeholders
# Usage: ./scripts/validate.sh

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🔍 Validating template placeholders..."
echo ""

# Track if any errors found
ERRORS=0

# Check for @yourusername
echo "Checking for '@yourusername' placeholders..."
if grep -r "@yourusername" . \
  --exclude-dir=.git \
  --exclude-dir=target \
  --exclude-dir=node_modules \
  --exclude-dir=dist \
  --exclude-dir=.claude \
  --exclude="*.lock" \
  --exclude="validate.sh" \
  2>/dev/null; then
  echo -e "${RED}❌ Found unreplaced '@yourusername' placeholders${NC}"
  ERRORS=$((ERRORS + 1))
else
  echo -e "${GREEN}✓ No '@yourusername' placeholders found${NC}"
fi
echo ""

# Check for template-mcp-server (excluding docs and this script)
echo "Checking for 'template-mcp-server' placeholders..."
TEMPLATE_REFS=$(grep -r "template-mcp-server" . \
  --exclude-dir=.git \
  --exclude-dir=target \
  --exclude-dir=node_modules \
  --exclude-dir=dist \
  --exclude-dir=.claude \
  --exclude="*.lock" \
  --exclude="validate.sh" \
  --exclude="init.sh" \
  2>/dev/null | grep -v "^Binary file" | grep -v "# Template" | grep -v "This template" || true)

if [ -n "$TEMPLATE_REFS" ]; then
  # Check if template-mcp-server directory still exists
  if [ -d "template-mcp-server" ]; then
    echo -e "${RED}❌ Found 'template-mcp-server' directory - needs to be renamed${NC}"
    ERRORS=$((ERRORS + 1))
  fi

  # Show non-documentation references
  FILTERED_REFS=$(echo "$TEMPLATE_REFS" | grep -v "README.md:" | grep -v "PUBLISHING.md:" | grep -v "CONTRIBUTING.md:" || true)
  if [ -n "$FILTERED_REFS" ]; then
    echo -e "${RED}❌ Found unreplaced 'template-mcp-server' references:${NC}"
    echo "$FILTERED_REFS"
    ERRORS=$((ERRORS + 1))
  else
    echo -e "${GREEN}✓ Only documentation references found (OK)${NC}"
  fi
else
  echo -e "${GREEN}✓ No 'template-mcp-server' placeholders found${NC}"
fi
echo ""

# Check for placeholder email
echo "Checking for placeholder email..."
if grep -r "your.email@example.com" . \
  --exclude-dir=.git \
  --exclude-dir=target \
  --exclude-dir=node_modules \
  --exclude-dir=dist \
  --exclude-dir=.claude \
  --exclude="validate.sh" \
  --exclude="init.sh" \
  2>/dev/null; then
  echo -e "${YELLOW}⚠️  Found placeholder email 'your.email@example.com'${NC}"
  echo -e "${YELLOW}   (This is OK if template not yet initialized)${NC}"
  ERRORS=$((ERRORS + 1))
else
  echo -e "${GREEN}✓ No placeholder email found${NC}"
fi
echo ""

# Check for placeholder author name
echo "Checking for placeholder author name..."
if grep -r "Your Name" . \
  --exclude-dir=.git \
  --exclude-dir=target \
  --exclude-dir=node_modules \
  --exclude-dir=dist \
  --exclude-dir=.claude \
  --exclude="validate.sh" \
  --exclude="init.sh" \
  2>/dev/null | grep -v "Your Name:" | grep -v "author:"; then
  echo -e "${YELLOW}⚠️  Found placeholder 'Your Name'${NC}"
  echo -e "${YELLOW}   (This is OK if template not yet initialized)${NC}"
  ERRORS=$((ERRORS + 1))
else
  echo -e "${GREEN}✓ No placeholder author name found${NC}"
fi
echo ""

# Check for yourusername in URLs
echo "Checking for 'yourusername' in repository URLs..."
if grep -r "github.com/yourusername" . \
  --exclude-dir=.git \
  --exclude-dir=target \
  --exclude-dir=node_modules \
  --exclude-dir=dist \
  --exclude-dir=.claude \
  --exclude="validate.sh" \
  --exclude="init.sh" \
  2>/dev/null; then
  echo -e "${RED}❌ Found 'yourusername' in GitHub URLs${NC}"
  ERRORS=$((ERRORS + 1))
else
  echo -e "${GREEN}✓ No 'yourusername' in URLs found${NC}"
fi
echo ""

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $ERRORS -eq 0 ]; then
  echo -e "${GREEN}✅ All validations passed!${NC}"
  echo ""
  echo "Your template appears to be properly initialized."
  exit 0
else
  echo -e "${RED}❌ Found $ERRORS validation issue(s)${NC}"
  echo ""
  echo "To fix these issues:"
  echo "  1. Run ./init.sh to initialize the template"
  echo "  2. Or manually replace the placeholders listed above"
  exit 1
fi
