#include <gtest/gtest.h>

#include "json_parser.h"

using namespace JsonParser;

TEST(JsonParserTest, TestJsonWithNoNests) {
  std::string json_text =
      "{ \"one\": 1, \n \"two\": 2.2, \n \"three\": 3.3 \n }";
  text_it start = json_text.begin();
  JsonValue parsed = ParseJsonHelper(json_text, start);
  EXPECT_EQ((*parsed.json)["one"].i, 1);
  EXPECT_EQ((*parsed.json)["two"].d, 2.2);
};

TEST(JsonParserTest, TestJsonWithNests) {
  std::string json_text =
      "{\n \"one\": 1,\n \"two\": {\n\"three\": 3, \n \"four\": { \n\"five\": 5 \n } \n }, \n \"six\": {\n\"seven\": 7\n } \n}";
  text_it start = json_text.begin();
  JsonValue parsed = ParseJsonHelper(json_text, start);

  JsonValue two = (*parsed.json)["two"];
  JsonValue four = (*two.json)["four"];
  JsonValue six = (*parsed.json)["six"];

  EXPECT_EQ((*parsed.json)["one"].i, 1);
  EXPECT_EQ((*two.json)["three"].i, 3);
  EXPECT_EQ((*four.json)["five"].i, 5);
  EXPECT_EQ((*six.json)["seven"].i, 7);
};
