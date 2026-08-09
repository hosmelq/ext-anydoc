param(
    [string] $PackageVersion
)

$ErrorActionPreference = 'Stop'

$metadata = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json
$package = $metadata.packages | Where-Object { $_.name -eq 'ext-anydoc' }

if ($null -eq $package) {
    throw 'Could not find the ext-anydoc package in Cargo metadata.'
}

$expectedPackageVersion = "v$($package.version)"

if ([string]::IsNullOrWhiteSpace($PackageVersion)) {
    $PackageVersion = $expectedPackageVersion
}

if ($PackageVersion -cne $expectedPackageVersion) {
    throw "Release tag $PackageVersion does not match Cargo version $($package.version)."
}

$phpVersion = php -r "echo PHP_MAJOR_VERSION, '.', PHP_MINOR_VERSION;"
$threadSafety = php -r "echo PHP_ZTS ? 'ts' : 'nts';"
$architecture = php -r "echo PHP_INT_SIZE === 8 ? 'x86_64' : 'x86';"
$phpInfo = php -i | Out-String
$compilerMatch = [regex]::Match($phpInfo, '(?im)^PHP Extension Build\s*=>\s*.*?(VS\d+)')

if ($architecture -ne 'x86_64') {
    throw "Unsupported Windows architecture: $architecture."
}

if (-not $compilerMatch.Success) {
    throw 'Could not determine the compiler used by PHP.'
}

$compiler = $compilerMatch.Groups[1].Value.ToLowerInvariant()

cargo build --locked --release

if ($LASTEXITCODE -ne 0) {
    throw 'Cargo failed to build the extension.'
}

$dllPath = Resolve-Path 'target/release/anydoc.dll'

php -n -d "extension=$dllPath" -r "exit(extension_loaded('anydoc') ? 0 : 1);"

if ($LASTEXITCODE -ne 0) {
    throw 'PHP failed to load the Anydoc extension.'
}

$testRunner = Join-Path $env:RUNNER_TEMP "run-tests-$phpVersion.php"

Invoke-WebRequest `
    -Uri "https://raw.githubusercontent.com/php/php-src/PHP-$phpVersion/run-tests.php" `
    -OutFile $testRunner

$env:REPORT_EXIT_STATUS = '1'
php $testRunner -P -q -n -d "extension=$dllPath" tests

if ($LASTEXITCODE -ne 0) {
    throw 'The PHPT test suite failed.'
}

$archiveBase = "php_anydoc-$PackageVersion-$phpVersion-$threadSafety-$compiler-x86_64"
$distPath = Join-Path $PWD 'dist'
$packagedDllPath = Join-Path $distPath "$archiveBase.dll"
$packagedLicensePath = Join-Path $distPath 'LICENSE.md'
$archivePath = Join-Path $distPath "$archiveBase.zip"

New-Item -ItemType Directory -Path $distPath -Force | Out-Null
Copy-Item -LiteralPath $dllPath -Destination $packagedDllPath -Force
Copy-Item -LiteralPath 'LICENSE.md' -Destination $packagedLicensePath -Force
Compress-Archive `
    -LiteralPath $packagedDllPath, $packagedLicensePath `
    -DestinationPath $archivePath `
    -Force

Add-Type -AssemblyName System.IO.Compression.FileSystem
$archive = [System.IO.Compression.ZipFile]::OpenRead($archivePath)

try {
    $entries = @($archive.Entries.FullName | Sort-Object)
    $expectedEntries = @(
        'LICENSE.md'
        "$archiveBase.dll"
    )

    if (Compare-Object -ReferenceObject $expectedEntries -DifferenceObject $entries) {
        throw 'The Windows package does not contain the expected files.'
    }
} finally {
    $archive.Dispose()
}

$resolvedArchivePath = (Resolve-Path $archivePath).Path

if (-not [string]::IsNullOrWhiteSpace($env:GITHUB_OUTPUT)) {
    "archive=$resolvedArchivePath" | Out-File `
        -FilePath $env:GITHUB_OUTPUT `
        -Append `
        -Encoding utf8
}

Write-Host "Created $resolvedArchivePath"
