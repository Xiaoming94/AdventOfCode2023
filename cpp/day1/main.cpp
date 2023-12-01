#include "problem1.h"
#include <iostream>

#include <utils.h>

void solveProblem1(const std::string& input)
{
    const auto results = reconstructCalibrationDocument(input);
    std::cout << "Solution for problem 1: " << results << std::endl;
}

int main()
{
    const auto input = readInput();
    solveProblem1(input);
    return 0;
}