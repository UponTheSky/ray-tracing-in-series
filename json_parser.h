#include <string>
#include <stdlib.h>
#include <memory>

#include <fstream>

namespace JsonParser {
  using text_it = std::string::iterator;

  union JsonValue {
    int i;
    double d;
    JsonValue* json = nullptr;
  };

  std::string& ReadFile(std::string filepath, std::string& output);

  JsonValue ParsePrimitive(const std::string& text, text_it start, text_it end);

  JsonValue ParseJsonValue();
}
