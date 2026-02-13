#!/bin/bash
echo "=== VeriLLM Environment Verification ==="
echo ""
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'
check_command() {
if command -v $1 &> /dev/null; then
echo -e "${GREEN}✅${NC} $1 is installed"
$1 --version 2>&1 | head -n1
else
echo -e "${RED}❌${NC} $1 is NOT installed"
fi
echo ""
}
check_command git
check_command rustc
check_command cargo
check_command docker
check_command kubectl
check_command minikube
check_command helm
check_command node
check_command npm
echo "Checking Docker..."
if docker info &> /dev/null; then
echo -e "${GREEN}✅${NC} Docker is running"
else
echo -e "${RED}❌${NC} Docker is NOT running"
fi
echo ""
echo "=== Verification Complete ==="
