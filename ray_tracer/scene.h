#include <iostream>

#include "common/color.h"
#include "camera/camera.h"
#include "object/object.h"

class scene {
  public:
    scene() = default;
    scene(
      // member data
      size_t image_width,
      size_t samples_per_pixel,
      color& background,
      int max_depth,
      // camera parameters
      point3& lookfrom,
      point3& lookat,
      vec3& vup,
      double vfov,
      double aspect_ratio,
      double aperture,
      double dist_to_focus,
      double time0,
      double time1
    ) : image_width(image_width),
        image_height(int(image_width / aspect_ratio)),
        samples_per_pixel(samples_per_pixel),
        background(background),
        max_depth(max_depth)
    {
      cam = create_camera(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus, time0, time1);
    }

    using scene_generator = hittable_list (*)();

    void render(scene_generator func);

  private:
    // hyper paramters for rendering
    size_t image_width;
    size_t image_height;
    size_t samples_per_pixel;
    color background;
    int max_depth;

    // camera
    camera cam;
    static camera create_camera(
      point3 lookfrom,
      point3 lookat,
      vec3 vup,
      double vfov,
      double aspect_ratio,
      double aperture,
      double focus_dist,
      double _time0 = 0,
      double _time1 = 0
    );
};

color ray_color(const ray& r, const color& background, const hittable& world, int depth);

hittable_list two_spheres();
hittable_list random_scene();
hittable_list two_perlin_spheres();
hittable_list earth();
hittable_list simple_light();
hittable_list cornell_box();
hittable_list cornell_smoke();
