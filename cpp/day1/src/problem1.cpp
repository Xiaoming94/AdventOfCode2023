#include "problem1.h"
#include <vector>
#include <numeric>
#include <sstream>
#include <algorithm>
#include <regex>
#include <iostream>

namespace{
    std::uint8_t wordToDigit(const std::string& word)
    {
        if(word == "one")
        {
            return 1u;
        }

        if(word == "two")
        {
            return 2u;
        }
        
        if(word == "three")
        {
            return 3u;
        }
        
        if(word == "four")
        {
            return 4u;
        }
        
        if(word == "five")
        {
            return 5u;
        }
        
        if(word == "six")
        {
            return 6u;
        }
        
        if(word == "seven")
        {
            return 7u;
        }
        
        if(word == "eight")
        {
            return 8u;
        }
        
        if(word == "nine")
        {
            return 9u;
        }
        std::cout << "Couldn't match argument" << std::endl;
        return 10u;
    }

    std::uint8_t convertToNumber(const char character)
    {
        return character - '0';
    }

    std::uint8_t convertToNumber(const std::string& word)
    {
        if(word.length() > 1)
        {
            return wordToDigit(word);
        }
        return convertToNumber(word.at(0));
    }
    std::uint32_t reconstructCalibrationDocumentLine(const std::string& input)
    {
        const auto isDigit = [](const auto inChar){ return std::isdigit(inChar); };
        const auto firstDigit = std::find_if(input.begin(), input.end(), isDigit);
        const auto lastDigit = std::find_if(input.rbegin(), input.rend(), isDigit);
        return convertToNumber(*firstDigit) * 10 + convertToNumber(*lastDigit); 
    }

    std::uint32_t reconstructCalibrationDocumentLineV2(const std::string& input)
    {
        std::string inputCopy{input};
        std::vector <std::string> wordDigits;
        const std::regex matchRegex("(?=(one|two|three|two|three|four|five|six|seven|eight|nine|[0-9])).");
        for(std::smatch matches; std::regex_search(inputCopy, matches, matchRegex);)
        {
            wordDigits.push_back(matches.str(1));
            inputCopy = matches.suffix();
        }
    
        return convertToNumber(wordDigits.front())*10 + convertToNumber(wordDigits.back());
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
    const std::vector<std::string> inputLines = splitLines(input);
    return std::accumulate(inputLines.begin(),
                           inputLines.end(), 
                           0,
                           [](auto result, const auto& currentLine){
                               return result + reconstructCalibrationDocumentLineV2(currentLine);
                           });
}