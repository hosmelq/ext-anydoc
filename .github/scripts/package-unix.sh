#!/usr/bin/env bash

set -euo pipefail

package_version="${1:-}"

if [[ -z "${package_version}" ]]; then
  echo "A package version is required."
  exit 1
fi

cargo_version="$(
  cargo metadata --locked --format-version 1 --no-deps |
    jq --raw-output '.packages[] | select(.name == "ext-anydoc") | .version'
)"
expected_package_version="v${cargo_version}"

if [[ "${package_version}" != "${expected_package_version}" ]]; then
  echo "Release tag ${package_version} does not match Cargo version ${cargo_version}."
  exit 1
fi

php_version="$(php -r "echo PHP_MAJOR_VERSION, '.', PHP_MINOR_VERSION;")"
thread_safety="$(php -r "echo PHP_ZTS ? 'zts' : 'nts';")"

case "$(uname -m)" in
  aarch64 | arm64)
    architecture="arm64"
    ;;
  amd64 | x86_64)
    architecture="x86_64"
    ;;
  *)
    echo "Unsupported Unix architecture: $(uname -m)."
    exit 1
    ;;
esac

case "$(uname -s)" in
  Darwin)
    libc="bsdlibc"
    operating_system="darwin"
    ;;
  Linux)
    operating_system="linux"
    ldd_version="$(ldd --version 2>&1 || true)"

    if grep --ignore-case --quiet musl <<< "${ldd_version}"; then
      libc="musl"
    else
      libc="glibc"
    fi
    ;;
  *)
    echo "Unsupported Unix operating system: $(uname -s)."
    exit 1
    ;;
esac

module_path="${PWD}/pie/modules/anydoc.so"

if [[ ! -f "${module_path}" ]]; then
  echo "Built extension not found at ${module_path}."
  exit 1
fi

php -n -d "extension=${module_path}" -r "exit(extension_loaded('anydoc') ? 0 : 1);"

archive_base="php_anydoc-${package_version}_php${php_version}"
archive_base+="-${architecture}-${operating_system}-${libc}-${thread_safety}"
dist_path="${PWD}/dist"
archive_path="${dist_path}/${archive_base}.zip"

mkdir -p "${dist_path}"
zip -q -FS -j "${archive_path}" "${module_path}" LICENSE.md

entries="$(unzip -Z1 "${archive_path}" | sort)"
expected_entries="$(printf '%s\n' LICENSE.md anydoc.so | sort)"

if [[ "${entries}" != "${expected_entries}" ]]; then
  echo "Unix package ${archive_path} does not contain the expected files."
  exit 1
fi

if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  printf 'archive=%s\n' "${archive_path}" >> "${GITHUB_OUTPUT}"
fi

echo "Created ${archive_path}"
