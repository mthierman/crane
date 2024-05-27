# https://stackoverflow.com/questions/11497457/git-clone-without-git-directory

$json = Get-Content -Path ".\github.json" | ConvertFrom-Json

$json.PSObject.Properties.Value | ForEach-Object -Process {
    Write-Host $_.PSObject.Properties.Name
    Write-Host $_.PSObject.Properties.Value
    # $_.PSObject.Properties.Name
    # $_.PSObject.Properties.Value
    # $_.PSObject.Properties.Name
    # "https://github.com/$($_.PSObject.Properties.Name)/archive/refs/tags/$($_.PSObject.Properties.Value.tag).zip"
}

# $json.PSObject.Properties.Value

# foreach ($item in $json.PSObject.Properties.Value)
# {
#     $repo = $item.PSObject.Properties.Name
#     $strategy = $item.PSObject.Properties.Value.strategy
#     $tag = $item.PSObject.Properties.Value.tag
#     $url = "https://github.com/$repo/archive/refs/tags/$tag.zip"

#     $repo

#     $url
# }

# foreach ($repository in $json.repos)
# {
#     $repo = $obj.PSObject.Properties.Name
#     $strategy = $obj.PSObject.Properties.Value.strategy
#     $tag = $obj.PSObject.Properties.Value.tag
#     $url = "https://github.com/$repo/archive/refs/tags/$tag.zip"

#     $repo
#     $tag
#     $url

#     # Invoke-WebRequest -Uri "https://github.com/$repo/archive/refs/tags/$tag.zip" | ForEach-Object { [System.IO.Compression.ZipFile]::ExtractToDirectory($_.RawContentStream, "$repo/$tag") }
# }

# $tag = "v1.6.43"
# Invoke-WebRequest -Uri "https://github.com/pnggroup/libpng/archive/refs/tags/$tag.zip" | ForEach-Object { [System.IO.Compression.ZipFile]::ExtractToDirectory($_.RawContentStream, "libpng") }
