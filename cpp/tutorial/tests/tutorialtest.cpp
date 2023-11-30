#include "tutorial.h"

#include <gtest/gtest.h>



TEST(Tutorial, BasicTest)
{
    const auto results = tutorialFunction(42);
    EXPECT_EQ(results, 84);
}