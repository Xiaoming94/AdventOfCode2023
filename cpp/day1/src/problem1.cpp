#include "problem1.h"
#include <optional>
#include <vector>
#include <numeric>
#include <sstream>

namespace{
    std::optional<std::uint8_t> convertToNumber(const char character)
    {
        if(std::isdigit(character))
        {
            return character - '0';
        }
        else
        {
            return std::nullopt;
        }
    }

    bool isNotAssigned(std::uint32_t variable)
    {
        return variable > 9;
    }

    std::uint32_t reconstructCalibrationDocumentLine(const std::string& input)
    {
        auto first = 10u;
        auto last = 10u;
        for (const auto character : input)
        {
            if (const auto number = convertToNumber(character))
            {
                last = *number;
                if (isNotAssigned(first))
                {
                    first = *number;
                }
            }
        }
        return first * 10 + last;
    }

    std::vector<std::string> splitLines(const std::string& input)
    {
        std::vector<std::string> lines;
        std::stringstream inputStream(input);
        std::string line;
        while(std::getline(inputStream, line, '\n'))
        {
            lines.push_back(line);
        }
        return lines;
    }
}


std::uint32_t reconstructCalibrationDocument(const std::string& input)
{
    //split lines of Input
    //For each lines : reconstructCalibrationDocumentLine(line)
    const std::vector<std::string> inputLines = splitLines(input);
    return std::accumulate(inputLines.begin(),
                    inputLines.end(), 
                    0,
                    [](auto result, const auto& currentLine){
                        return result + reconstructCalibrationDocumentLine(currentLine);
                    });
}