#include <glow/glow.hxx>

#include <stdlib.h>

#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#include <nlohmann/json.hpp>

using json = nlohmann::json;

struct Manifest {
    std::vector<std::string> packages;

    NLOHMANN_DEFINE_TYPE_INTRUSIVE(Manifest, packages);
};

auto main() -> int {
    // if (auto path { glow::filesystem::known_folder(FOLDERID_Downloads) }) {
    //     *path = *path / L"test.zip";

    //     glow::network::download_file(
    //         "https://github.com/mthierman/crane/archive/refs/heads/main.zip", *path);
    // }

    if (auto path { glow::filesystem::known_folder() }) {
        *path = *path / L"crane" / L"crane.json";

        if (auto stream { std::ifstream(*path) }; stream.is_open()) {
            auto manifest { json::parse(stream).get<Manifest>() };

            for (auto& package : manifest.packages) {
                std::cout << package << std::endl;
            }
        }
    }

    return EXIT_SUCCESS;
}
