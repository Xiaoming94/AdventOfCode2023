#include "problem1.h"

#include <gtest/gtest.h>

using namespace ::testing;

TEST(Tutorial, BasicTest)
{
    const auto results = tutorialFunction(42);
    EXPECT_EQ(results, 84);
}