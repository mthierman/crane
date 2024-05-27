$packages = Get-Content -Path ".\crane.json" | ConvertFrom-Json

foreach ($package in $packages)
{
    $user = $package.user
    $repo = $package.repo
    $tag = $package.tag

    $url = "https://github.com/$user/$repo/archive/refs/tags/$tag.zip"
    
    Invoke-WebRequest -Uri $url | ForEach-Object { [System.IO.Compression.ZipFile]::ExtractToDirectory($_.RawContentStream, "libs/$user") }
}
