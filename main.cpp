#include "ray_tracer/utils/json_parser.h"
#include "ray_tracer/scene.h"

using JsonMap = std::map<std::string, JsonParser::JsonValue>;

int main() {
  // parse the config
  JsonParser::JsonValue config = JsonParser::ParseJson("config.json");
  JsonMap config_json = (*config.json);

  // member data
  size_t image_width = (size_t)(config_json["image_width"].i);
  size_t samples_per_pixel = (size_t)(config_json["samples_per_pixel"].i);

  JsonMap bgcolor_json = (*config_json["background"].json);
  color background((double)(bgcolor_json["r"].i),  (double)(bgcolor_json["g"].i), (double)(bgcolor_json["b"].i));
  int max_depth = config_json["max_depth"].i;

  // camera parameters
  JsonMap lookfrom_json = (*config_json["lookfrom"].json);
  point3 lookfrom((double(lookfrom_json["x"].i)), (double(lookfrom_json["y"].i)), -(double(lookfrom_json["z"].i)));

  JsonMap lookat_json = (*config_json["lookat"].json);
  point3 lookat((double(lookat_json["x"].i)), (double(lookat_json["y"].i)), (double(lookat_json["z"].i)));

  JsonMap vup_json = (*config_json["vup"].json);
  vec3 vup((double(vup_json["x"].i)), (double(vup_json["y"].i)), (double(vup_json["z"].i)));

  double vfov = config_json["vfov"].d;
  double aspect_ratio = config_json["aspect_ratio"].d;
  double aperture = config_json["aperture"].d;
  double dist_to_focus = config_json["dist_to_focus"].d;
  double time0 = config_json["time0"].d;
  double time1 = config_json["time1"].d;

  // generate the scene
  scene current_scene(
    image_width,
    samples_per_pixel,
    background,
    max_depth,
    lookfrom,
    lookat,
    vup,
    vfov,
    aspect_ratio,
    aperture,
    dist_to_focus,
    time0,
    time1
  );

  // render
  current_scene.render(cornell_box);

  return 0;
}
