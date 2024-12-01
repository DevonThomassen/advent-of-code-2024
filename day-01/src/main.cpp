#include <iostream>
#include <fstream>
#include <filesystem>
#include <sstream>
#include <vector>
#include <algorithm>

static constexpr auto FILE_PATH = "resources/input.txt";

int main()
{
    std::vector<int> left_list;
    std::vector<int> right_list;

    std::ifstream example(FILE_PATH);
    std::string line;
    while (std::getline(example, line))
    {
        std::stringstream ss(line);
        int left = -1;
        int right = -1;

        if (!(ss >> left >> right))
        {
            std::cerr << "Error \n";
            continue;
        }
        left_list.push_back(left);
        right_list.push_back(right);
    }
    example.close();

    std::ranges::sort(left_list);
    std::ranges::sort(right_list);

    auto total_distance = 0;
    for (auto index = 0; index < left_list.size(); ++index)
    {
        const auto distance = std::abs(left_list[index] - right_list[index]);
        total_distance += distance;
    }

    std::cout << total_distance << "\n";
    return 0;
}
