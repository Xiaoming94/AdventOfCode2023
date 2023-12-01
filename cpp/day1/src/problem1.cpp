#include "problem1.h"
#include <vector>
#include <numeric>
#include <sstream>
#include <algorithm>

namespace{
    std::uint8_t convertToNumber(const char character)
    {
        return character - '0';
    }

    std::uint32_t reconstructCalibrationDocumentLine(const std::string& input)
    {
        const auto isDigit = [](const auto inChar) -> bool { return std::isdigit(inChar); };
        const auto firstDigit = std::find_if(input.begin(), input.end(), isDigit);
        const auto lastDigit = std::find_if(input.rbegin(), input.rend(), isDigit);
        return convertToNumber(*firstDigit) * 10 + convertToNumber(*lastDigit); 
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

std::uint32_t reconstructCalibrationDocumentV2(const std::string &input)
{
    return 0;
}