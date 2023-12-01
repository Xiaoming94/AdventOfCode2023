#include "problem1.h"

#include <gtest/gtest.h>

using namespace ::testing;

TEST(AdventOfCodeDay1Test, verify_sumOfNumbersInLines)
{
    constexpr auto input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    EXPECT_EQ(142, reconstructCalibrationDocument(input));
}

TEST(AdventOfCodeDay1Test, verify_1Line)
{
    constexpr auto input{"11"};
    EXPECT_EQ(11, reconstructCalibrationDocument(input));
}

TEST(AdventOfCodeDay1Test, verify_1LineWithChars)
{
    constexpr auto input{"1abc2"};
    EXPECT_EQ(12, reconstructCalibrationDocument(input));
}

TEST(AdventOfCodeDay1Test, verify_1LineWithMultipleNumbers)
{
    constexpr auto input{"1a5c2"};
    EXPECT_EQ(12, reconstructCalibrationDocument(input));
}

TEST(AdventOfCodeDay1Test, verify_1LineWithOnlyMultipleNumbers)
{
    constexpr auto input{"123456"};
    EXPECT_EQ(16, reconstructCalibrationDocument(input));
}

TEST(AdventOfCodeDay1Test, verify_1LineWithOnly1Number)
{
    constexpr auto input{"a1c"};
    EXPECT_EQ(11, reconstructCalibrationDocument(input));
}

TEST(AdventOfCodeDay1Test, verify_1LineOnly1Digit)
{
    constexpr auto input{"1"};
    EXPECT_EQ(11, reconstructCalibrationDocument(input));
}

TEST(AdventOfCodeDay1Test, verify_2Line)
{
    constexpr auto input{"ab1c2\ngasd26"};
    EXPECT_EQ(38, reconstructCalibrationDocument(input));
}

TEST(AdventOfCodeDay1Test, verify_2LineWithSpace)
{
    constexpr auto input{"ab1c2\ngasd2 6"};
    EXPECT_EQ(38, reconstructCalibrationDocument(input));
}