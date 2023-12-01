#include "readinput.h"
#include <iostream>

std::string readInput()
{
    std::string line;
    std::string ret;
    while(std::getline(std::cin, line, '\n'))
    {
        ret += line;
        ret += "\n";
    }
    return ret;
}