# https://stackoverflow.com/questions/11497457/git-clone-without-git-directory

$json = Get-Content -Path ".\github.json" | ConvertFrom-Json

foreach ($repo in $json.PSObject.Properties.Value)
{
    # $repo
    foreach ($repo in $repo.PSObject.Properties)
    {
        Write-Host $repo.Name
        Write-Host $repo.Value.tag
        Write-Host "https://github.com/$($repo.Name)/archive/refs/tags/$($repo.Value.tag).zip"

        Invoke-WebRequest -Uri "https://github.com/$($repo.Name)/archive/refs/tags/$($repo.Value.tag).zip" | ForEach-Object { [System.IO.Compression.ZipFile]::ExtractToDirectory($_.RawContentStream, "libs/$($repo.Name)/$($repo.Value.tag)") }
    }
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
