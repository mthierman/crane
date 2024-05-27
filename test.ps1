# https://stackoverflow.com/questions/11497457/git-clone-without-git-directory

$json = Get-Content -Path ".\github.json" | ConvertFrom-Json
# $json | Get-Member
# $json.repos.'pnggroup/libpng'
# $json | ForEach-Object { $_.repos }
# $json.repos | ForEach-Object { 
#     $_
#     # Invoke-WebRequest -Uri "https://github.com/$_/archive/refs/tags/$tag.zip" | ForEach-Object { [System.IO.Compression.ZipFile]::ExtractToDirectory($_.RawContentStream, "libpng") }
# } | Format-List

# $json.repos.PSObject.Properties

foreach ($obj in $json.repos)
{
    $obj.PSObject.Properties.Name
    # $obj.PSObject.Properties.Value
    $obj.PSObject.Properties.Value.strategy
    $obj.PSObject.Properties.Value.tag
}

# $tag = "v1.6.43"
# Invoke-WebRequest -Uri "https://github.com/pnggroup/libpng/archive/refs/tags/$tag.zip" | ForEach-Object { [System.IO.Compression.ZipFile]::ExtractToDirectory($_.RawContentStream, "libpng") }
