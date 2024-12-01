#include <iostream>
#include <fstream>
#include <filesystem>
#include <sstream>
#include <vector>
#include <unordered_map>

static constexpr auto FILE_PATH = "resources/example.txt";

int main()
{
    std::vector<int> left_list;
    std::unordered_map<int, int> map;

    std::ifstream example(FILE_PATH);
    std::string line;
    while (std::getline(example, line))
    {
        std::stringstream ss(line);
        int left = -1;
        int right = -1;

        if (!(ss >> left >> right))
        {
            std::cerr << "Error reading line\n";
            continue;
        }
        left_list.push_back(left);
        ++map[right];
    }
    example.close();

    auto answer = 0;
    for (const auto& left : left_list)
    {
        if (auto it = map.find(left); it != map.end())
        {
            answer += left * it->second;
        }
    }

    std::cout << answer << "\n";
    return 0;
}
