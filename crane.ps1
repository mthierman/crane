$packages = Get-Content -Path ".\crane.json" | ConvertFrom-Json

if (Test-Path "libs") { Remove-Item "libs" -Force -Recurse }

foreach ($package in $packages)
{
    $split = $package -split ':', 2

    if ($split[0] -eq "gh")
    {
        $split = $package -split '[:/]'

        $crane = [pscustomobject]@{
            protocol = $split[0]
            user     = $split[1]
            repo     = $split[2]
            tag      = $split[3]
            file     = $split[4]
        }

        $crane | Format-List

        $tags = "https://github.com/$($crane.user)/$($crane.repo)/archive/refs/tags/$($crane.tag).zip"
        $heads = "https://github.com/$($crane.user)/$($crane.repo)/archive/refs/heads/$($crane.tag).zip"
        $releases = "https://github.com/$($crane.user)/$($crane.repo)/releases/download/$($crane.tag)/$($crane.file)"

        if ($crane.file -match "\S")
        {
            $request = Invoke-WebRequest -Uri $releases -SkipHttpErrorCheck

            if ($request.StatusCode -eq "200")
            {
                foreach ($response in $request)
                {
                    $response.StatusCode
                    [System.IO.Compression.ZipFile]::ExtractToDirectory($response.RawContentStream, "libs/$($crane.user)/$($crane.repo)")
                }
            }
        }
        else
        {
            $request = Invoke-WebRequest -Uri $tags -SkipHttpErrorCheck

            if ($request.StatusCode -eq "200")
            {
                foreach ($response in $request)
                {
                    $response.StatusCode
                    [System.IO.Compression.ZipFile]::ExtractToDirectory($response.RawContentStream, "libs/$($crane.user)")
                }
            }
            else
            {
                $request = Invoke-WebRequest -Uri $heads -SkipHttpErrorCheck

                if ($request.StatusCode -eq "200")
                {
                    foreach ($response in $request)
                    {
                        $response.StatusCode
                        [System.IO.Compression.ZipFile]::ExtractToDirectory($response.RawContentStream, "libs/$($crane.user)")
                    }
                }
            }
        }
    }
    else
    {
        $request = Invoke-WebRequest -Uri $split[1] -SkipHttpErrorCheck

        if ($request.StatusCode -eq "200")
        {
            foreach ($response in $request)
            {
                $response.StatusCode
                [System.IO.Compression.ZipFile]::ExtractToDirectory($response.RawContentStream, "libs/$($split[0])")
            }
        }
    }
}
