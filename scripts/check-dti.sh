#!/bin/bash

# ==============================================================================
# Define-Test-Implement (DTI) Methodology Checker
# Automatically verifies if a repository complies with the DTI standard.
# ==============================================================================

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🔍 Running DTI Methodology Compliance Check..."
echo "------------------------------------------------"

SCORE=0
MAX_SCORE=5

# 1. Check for Definition Template
if [ -f "templates/define.md" ]; then
    echo -e "${GREEN}✅ FOUND: Definition template (templates/define.md)${NC}"
    SCORE=$((SCORE+1))
else
    echo -e "${RED}❌ MISSING: Definition template (templates/define.md)${NC}"
fi

# 2. Check for Test Plan Template
if [ -f "templates/test-plan.md" ]; then
    echo -e "${GREEN}✅ FOUND: Test Plan template (templates/test-plan.md)${NC}"
    SCORE=$((SCORE+1))
else
    echo -e "${RED}❌ MISSING: Test Plan template (templates/test-plan.md)${NC}"
fi

# 3. Check for Github PR Template enforcing DTI
if [ -f ".github/pull_request_template.md" ]; then
    if grep -q "define.md" ".github/pull_request_template.md" && grep -q "test-plan.md" ".github/pull_request_template.md"; then
        echo -e "${GREEN}✅ FOUND: PR Template enforcing DTI artifacts (.github/pull_request_template.md)${NC}"
        SCORE=$((SCORE+1))
    else
        echo -e "${YELLOW}⚠️ WARNING: PR Template exists but doesn't explicitly mention define.md or test-plan.md${NC}"
    fi
else
    echo -e "${RED}❌ MISSING: GitHub PR Template (.github/pull_request_template.md)${NC}"
fi

# 4. Check for canonical methodology document and README link
README_FILE=""
if [ -f "readme.md" ]; then
    README_FILE="readme.md"
elif [ -f "README.md" ]; then
    README_FILE="README.md"
fi

if [ -f "standards/methodology.md" ]; then
    if [ -n "$README_FILE" ] && grep -q "standards/methodology.md" "$README_FILE"; then
        echo -e "${GREEN}✅ FOUND: Canonical methodology doc and README link (standards/methodology.md)${NC}"
        SCORE=$((SCORE+1))
    elif [ -n "$README_FILE" ]; then
        echo -e "${YELLOW}⚠️ WARNING: standards/methodology.md exists but README does not link to it${NC}"
    else
        echo -e "${YELLOW}⚠️ WARNING: standards/methodology.md exists but README.md file is missing${NC}"
    fi
else
    echo -e "${RED}❌ MISSING: Canonical methodology doc (standards/methodology.md)${NC}"
fi

# 5. Check Contributing Guidelines
if [ -f "standards/contributing.md" ]; then
    if grep -qi "Define" standards/contributing.md && grep -qi "Test" standards/contributing.md; then
         echo -e "${GREEN}✅ FOUND: Contributing guidelines enforce DTI phases${NC}"
         SCORE=$((SCORE+1))
    else
         echo -e "${YELLOW}⚠️ WARNING: Contributing.md does not explicitly enforce DTI phases${NC}"
    fi
else
    echo -e "${RED}❌ MISSING: Contributing guidelines (standards/contributing.md)${NC}"
fi

echo "------------------------------------------------"
echo -e "🏆 DTI Compliance Score: ${SCORE}/${MAX_SCORE}\n"

if [ $SCORE -eq $MAX_SCORE ]; then
    echo -e "${GREEN}🎉 Perfect! This repository fully enforces the DTI methodology.${NC}"
    exit 0
else
    echo -e "${RED}⚠️ This repository is missing structural DTI components. Fix the errors above to ensure team compliance.${NC}"
    exit 1
fi
