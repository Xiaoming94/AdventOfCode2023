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
                            std::make_pair("11", 11),
                            std::make_pair("1abc2", 12),
                            std::make_pair("1a5c2", 12),
                            std::make_pair("123456", 16),
                            std::make_pair("a1c", 11),
                            std::make_pair("1", 11),
                            std::make_pair("ab1c2\ngasd26", 38),
                            std::make_pair("ab1c2\ngasd2 6", 38)
                        ));
