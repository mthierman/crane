$packages = Get-Content -Path ".\test.json" | ConvertFrom-Json

foreach ($package in $packages)
{
    $split = $package -split '[@/]'

    $crane = [pscustomobject]@{
        user = $split[0]
        repo = $split[1]
        tag  = $split[2]
    }

    $obj | Format-List

    $url = "https://github.com/$user/$repo/archive/refs/tags/$tag.zip"
    
    Invoke-WebRequest -Uri $url | ForEach-Object { [System.IO.Compression.ZipFile]::ExtractToDirectory($_.RawContentStream, "libs/$user") }
}
