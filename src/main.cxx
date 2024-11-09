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

    NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Manifest, packages);
};

auto main() -> int {
    // if (auto path { glow::filesystem::known_folder(FOLDERID_Downloads) }) {
    //     *path = *path / L"test.zip";

    //     glow::network::download_file(
    //         "https://github.com/mthierman/crane/archive/refs/heads/main.zip", *path);
    // }

    //

    if (auto path { glow::filesystem::known_folder() }) {
        *path = *path / L"crane" / L"crane.json";

        auto stream { std::ifstream(*path) };

        if (stream.is_open()) {
            std::cout << "Reading crane.json..." << std::endl;
            auto data = json::parse(stream);
            Manifest manifest;
            manifest.packages = data["packages"].get<std::vector<std::string>>();

            for (auto& package : manifest.packages) {
                std::cout << package << std::endl;
            }

            // json j = manifest;
            // auto manifest { j.get<Manifest>() };

            // manifest = data.get<Manifest>();

            // auto packages { data.get<std::vector<std::string>>() };
            // for (auto& package&)
            // std::cout << std::setw(4) << data << std::endl;
        }
    }

    return EXIT_SUCCESS;
}
