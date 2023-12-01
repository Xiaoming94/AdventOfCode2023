#include "problem1.h"

#include <gtest/gtest.h>
#include <utility>

using namespace ::testing;

using input_t = std::string;
using result_t = std::uint32_t;

class AdventOfCodeDay1Problem1Test : public TestWithParam<std::pair<input_t, result_t>>
{
public:
    AdventOfCodeDay1Problem1Test()
    {

    }
};

TEST_F(AdventOfCodeDay1Problem1Test, verify_sumOfNumberLines)
{
    constexpr auto input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    EXPECT_EQ(142, reconstructCalibrationDocument(input));
}

TEST_P(AdventOfCodeDay1Problem1Test, unitTest_verifySolutionWithParams)
{
    const auto& [input, result] = GetParam();
    EXPECT_EQ(result, reconstructCalibrationDocument(input));
}

INSTANTIATE_TEST_CASE_P(AdventOfCodeDay1Problem1,
                        AdventOfCodeDay1Problem1Test,
                        Values(
                            std::make_pair("11",11)
                        ));

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