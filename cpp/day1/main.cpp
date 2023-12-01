#include "problem1.h"
#include <iostream>

#include <utils.h>

int main()
{
    const auto input = readInput();
    const auto results = reconstructCalibrationDocument(input);
    std::cout << "Reconstructed Calibration Number: " << results << std::endl;
    return 0;
}