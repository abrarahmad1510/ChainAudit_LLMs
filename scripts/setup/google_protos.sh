#!/bin/bash
set -e
mkdir -p src/shared/proto/google/rpc
mkdir -p src/shared/proto/google/api
curl -o src/shared/proto/google/rpc/status.proto https://raw.githubusercontent.com/googleapis/googleapis/master/google/rpc/status.proto
curl -o src/shared/proto/google/api/annotations.proto https://raw.githubusercontent.com/googleapis/googleapis/master/google/api/annotations.proto
curl -o src/shared/proto/google/api/http.proto https://raw.githubusercontent.com/googleapis/googleapis/master/google/api/http.proto
echo "Google protos downloaded to src/shared/proto/google"
