#include <glow/glow.hxx>

#include <stdlib.h>

auto main() -> int {
    if (auto path { glow::filesystem::known_folder(FOLDERID_Downloads) }) {
        *path = *path / L"test.zip";

        glow::network::download_file(
            "https://github.com/mthierman/crane/archive/refs/heads/main.zip", *path);
    }

    return EXIT_SUCCESS;
}
