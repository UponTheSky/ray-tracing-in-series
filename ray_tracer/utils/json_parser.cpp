#include <algorithm> // for std::find
#include <fstream> // for std::ifstream
#include <cassert> // for assert()
#include <cctype> // for isdigit()

#include "json_parser.h"


using namespace JsonParser;

JsonValue JsonParser::ParseJson(const std::string& filepath) {
  // 1. read the text data from the given file
  std::string text;
  ReadFile(filepath, text);

  // 2. parse the text with the helper function and return
  text_it start = text.begin();
  return ParseJsonHelper(text, start);
}

void JsonParser::ReadFile(const std::string& filepath, std::string& output) {
  std::ifstream file(filepath);
  std::string line;

  while (std::getline(file, line)) {
    output.append(line); // append() copies the argument passed as a reference(&std::string)
  }
}

JsonValue JsonParser::ParsePrimitive(const std::string& text, text_it start, text_it end) {
  std::string substr = text.substr(start - text.begin(), end - start);
  size_t float_point_index = substr.find(".");

  if (float_point_index >= (end - start)) { // integer
    return {.i = std::stoi(substr)};
  } else { // float(double)
    return {.d = std::stod(substr) };
  }
}

std::pair<std::string, JsonValue> JsonParser::RetriveKeyValuePair(
  const std::string& text,
  text_it& it
) {
  assert(it != text.end());

  // ignore white spaces & line breaks
  while (*it == ' ' || *it == '\n') {
    it++;
  }

  text_it curr_it;
  std::string key;
  JsonValue value;
  // if hit a double quote for the first time, it is a key
  if (*it == '\"') {
    curr_it = ++it;
    while (*it != '\"') {
      it++;
    }

    key = text.substr(curr_it - text.begin(), it - curr_it);
    assert(*(++it) == ':'); // assert that we are parsing the key string
    it++;
  }

  // now we need to have its corresponding value
  while (*it == ' ' || *it == '\n') {
    it++;
  }

  if (*it == '{') {
    // another json format
    value = ParseJsonHelper(text, it);
  } else {
    // primitive value(double or int)
    curr_it = it;
    while (isdigit(*it) || *it == '.') {
      it++;
    }
    value = ParsePrimitive(text, curr_it, it);
  }

  // after parsing the value, check whether the current iterator points to a comma
  if (*it == ',') {
    it++;
  }

  return std::make_pair(key, value);
}

JsonValue JsonParser::ParseJsonHelper(const std::string& text, text_it& it) {
  assert(*it == '{'); // must start with the left curly bracket
  it++;

  std::map<std::string, JsonValue>* json_map = new std::map<std::string, JsonValue>;

  do {
    const auto [key, value] = RetriveKeyValuePair(text, it);
    (*json_map)[key] = value;

    while (*it == ' ' || *it == '\n') {
      it++;
    }
  } while (*it != '}');

  it++; // after '}'

  return { .json = json_map };
}
