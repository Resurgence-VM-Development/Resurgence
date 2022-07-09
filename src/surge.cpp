#include <vector>
#include <string_view>
#include <cstdlib>

#include "Resurgence.hpp"

int main(int argc, char** argv)
{
    const std::vector<std::string_view> Arguments (argv + 1, argv + argc);
    const ResurgenceVM Instance{Arguments};
    const bool Success = Instance.start();

    if (!Success) [[unlikely]]
        return EXIT_FAILURE;

    return EXIT_SUCCESS;
}