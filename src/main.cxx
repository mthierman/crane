#include <glow/glow.hxx>

#include <stdlib.h>

#include <fstream>
#include <iostream>

#include <nlohmann/json.hpp>

using json = nlohmann::json;

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
            std::cout << std::setw(4) << data << std::endl;
        }
    }

    return EXIT_SUCCESS;
}
