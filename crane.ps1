$packages = Get-Content -Path ".\crane.json" | ConvertFrom-Json

if (Test-Path "libs") { Remove-Item "libs" -Force -Recurse }

foreach ($package in $packages)
{
    $split = $package -split '[:/@]'

    $crane = [pscustomobject]@{
        user = $split[0]
        repo = $split[1]
        tag  = $split[2]
        file = $split[3]
    }

    $crane | Format-List

    $tags = "https://github.com/$($crane.user)/$($crane.repo)/archive/refs/tags/$($crane.tag).zip"
    $heads = "https://github.com/$($crane.user)/$($crane.repo)/archive/refs/heads/$($crane.tag).zip"
    $releases = "https://github.com/$($crane.user)/$($crane.repo)/releases/download/$($crane.tag)/$($crane.file).zip"

    $request = Invoke-WebRequest -Uri $tags -SkipHttpErrorCheck

    if ($crane.file) { $crane.file }
    else { $crane.repo }

    # if ($request.StatusCode -eq "200")
    # {
    #     foreach ($response in $request)
    #     {
    #         $response.StatusCode
    #         [System.IO.Compression.ZipFile]::ExtractToDirectory($response.RawContentStream, "libs/$($crane.user)")
    #     }
    # }
    # else
    # {
    #     $request = Invoke-WebRequest -Uri $heads -SkipHttpErrorCheck

    #     if ($request.StatusCode -eq "200")
    #     {
    #         foreach ($response in $request)
    #         {
    #             $response.StatusCode
    #             [System.IO.Compression.ZipFile]::ExtractToDirectory($response.RawContentStream, "libs/$($crane.user)")
    #         }
    #     }
    #     else
    #     {
    #         $request = Invoke-WebRequest -Uri $releases -SkipHttpErrorCheck
    #         if ($request.StatusCode -eq "200")
    #         {
    #             foreach ($response in $request)
    #             {
    #                 $response.StatusCode
    #                 [System.IO.Compression.ZipFile]::ExtractToDirectory($response.RawContentStream, "libs/$($crane.user)/$($crane.repo)")
    #             }
    #         }
    #         # gh release download -p "$($crane.tag)" -R "$($crane.user)/$($crane.repo)" --clobber
    #         # [System.IO.Compression.ZipFile]::ExtractToDirectory($($crane.tag), "libs/$($crane.user)/$($crane.repo)")
    #         # if (Test-Path "$($crane.tag)") { Remove-Item -Path "$($crane.tag)" -Force }
    #     }
    # }
}
