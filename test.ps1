# https://stackoverflow.com/questions/11497457/git-clone-without-git-directory

$json = Get-Content -Path ".\github.json" | ConvertFrom-Json
# $json | Get-Member
$json.repos.'pnggroup/libpng'

# $tag = "v1.6.43"
# Invoke-WebRequest -Uri "https://github.com/pnggroup/libpng/archive/refs/tags/$tag.zip" | ForEach-Object { [System.IO.Compression.ZipFile]::ExtractToDirectory($_.RawContentStream, "libpng") }
