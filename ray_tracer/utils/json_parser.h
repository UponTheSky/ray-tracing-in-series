#ifndef JSON_PARSER_H
#define JSON_PARSER_H

#include <string>
#include <map>
#include <utility> // for std::pair

namespace JsonParser {
  using text_it = std::string::iterator;

  /**
   * @brief A union value representing the value part of (key, value) pair.
   */
  union JsonValue {
    int i;
    double d;
    std::map<std::string, JsonValue>* json;
  };

  void ReadFile(const std::string& filepath, std::string& output);

  JsonValue ParsePrimitive(const std::string& text, text_it start, text_it end);

  std::pair<std::string, JsonValue> RetriveKeyValuePair(
    const std::string& text,
    text_it& it
  );

  JsonValue ParseJsonHelper(const std::string& text, text_it& it);

  JsonValue ParseJson(const std::string& filepath);
}

#endif
