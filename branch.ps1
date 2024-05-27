$request = Invoke-WebRequest -Uri "https://github.com/nlohmann/json/archive/refs/tags/develop.zip" -SkipHttpErrorCheck

if ($request.StatusCode -eq "404") { $request = Invoke-WebRequest -Uri "https://github.com/nlohmann/json/archive/refs/heads/develop.zip" }

if ($request.StatusCode -ne "404") { "Success!" }
