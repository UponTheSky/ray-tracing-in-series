#include <algorithm>
#include <fstream>
#include <cmath>

#include "json_parser.h"

using namespace JsonParser;

std::string& JsonParser::ReadFile(std::string filepath, std::string& output) {
  std::ifstream file(filepath);
  std::string line;

  while (std::getline(file, line)) {
    output.append(line); // append() copies the argument passed as a reference(&std::string)
  }

  return output;
}

JsonValue JsonParser::ParsePrimitive(const std::string& text, text_it start, text_it end) {

  std::string substr = text.substr(0, end - start);
  size_t float_point_index = substr.find(".");
  if (float_point_index >= (end - start)) { // integer
    return {.i = std::stoi(substr)};
  } else { // float(double)
    return {.d = std::stod(substr) };
  }
}
