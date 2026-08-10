#!/usr/bin/env bash

set -euo pipefail

release_tag="${1:-}"

if [[ -z "${release_tag}" ]]; then
  echo "A release tag is required."
  exit 1
fi

archives=(
  "php_anydoc-${release_tag}-8.4-nts-vs17-x86_64.zip"
  "php_anydoc-${release_tag}-8.4-ts-vs17-x86_64.zip"
  "php_anydoc-${release_tag}-8.5-nts-vs17-x86_64.zip"
  "php_anydoc-${release_tag}-8.5-ts-vs17-x86_64.zip"
  "php_anydoc-${release_tag}_php8.4-arm64-darwin-bsdlibc-nts.zip"
  "php_anydoc-${release_tag}_php8.4-arm64-linux-glibc-nts.zip"
  "php_anydoc-${release_tag}_php8.4-arm64-linux-glibc-zts.zip"
  "php_anydoc-${release_tag}_php8.4-x86_64-darwin-bsdlibc-nts.zip"
  "php_anydoc-${release_tag}_php8.4-x86_64-linux-glibc-nts.zip"
  "php_anydoc-${release_tag}_php8.4-x86_64-linux-glibc-zts.zip"
  "php_anydoc-${release_tag}_php8.5-arm64-darwin-bsdlibc-nts.zip"
  "php_anydoc-${release_tag}_php8.5-arm64-linux-glibc-nts.zip"
  "php_anydoc-${release_tag}_php8.5-arm64-linux-glibc-zts.zip"
  "php_anydoc-${release_tag}_php8.5-x86_64-darwin-bsdlibc-nts.zip"
  "php_anydoc-${release_tag}_php8.5-x86_64-linux-glibc-nts.zip"
  "php_anydoc-${release_tag}_php8.5-x86_64-linux-glibc-zts.zip"
)

printf '%s\n' "${archives[@]}"
