#include "problem1.h"

#include <gtest/gtest.h>

using namespace ::testing;

TEST(AdventOfCodeDay1Test, verify_sumOfNumbersInLines)
{
    constexpr auto input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    EXPECT_EQ(142, reconstructCalibrationDocument(input));
}