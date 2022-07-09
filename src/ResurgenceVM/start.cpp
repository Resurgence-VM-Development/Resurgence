#include <fmt/core.h>
#include "Resurgence.hpp"

[[nodiscard]] bool ResurgenceVM::start() const noexcept
{
    try 
    {
        
    }
    catch (std::exception& exp)
    {
        fmt::print("{}\n", exp.what());
        return false;
    }
    return true;
}