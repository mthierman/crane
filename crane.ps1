# https://stackoverflow.com/questions/11497457/git-clone-without-git-directory

$crane = Get-Content -Path ".\crane.json" | ConvertFrom-Json

foreach ($package in $crane.PSObject.Properties.Value)
{
    $user = $package.user
    $repo = $package.repo
    $tag = $package.tag

    $url = "https://github.com/$user/$repo/archive/refs/tags/$tag.zip"

    Invoke-WebRequest -Uri $url | ForEach-Object { [System.IO.Compression.ZipFile]::ExtractToDirectory($_.RawContentStream, "libs/$user") }
}
