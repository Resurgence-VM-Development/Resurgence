#pragma once

#include <vector>
#include <string_view>

class ResurgenceVM
{
    public:
        explicit ResurgenceVM(const std::vector<std::string_view>& t_Arguments) noexcept : Arguments(t_Arguments){}
        [[nodiscard]] bool start() const noexcept;

    private:
        const std::vector<std::string_view>& Arguments;
};