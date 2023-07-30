#ifndef SCENE_H
#define SCENE_H

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

    void render(scene_generator func) {
      hittable_list world = func();
      std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

      for (int j = image_height - 1; j >= 0; j--) {
        std::cerr << "\rScanlines remaining: " << j << ' ' << std::flush;
        for (int i = 0; i < image_width; i++) {
          color pixel_color(0, 0, 0);
          for (int s = 0; s < samples_per_pixel; ++s) {
            double u = double(i + random_double()) / (image_width - 1);
            double v = double(j + random_double()) / (image_height - 1);
            ray r = cam.get_ray(u, v);
            pixel_color += ray_color(r, background, world, max_depth);
          }
          write_color(std::cout, pixel_color, samples_per_pixel);
        }
      }
      std::cerr << "\nDone.\n";
    }

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
      double _time0,
      double _time1
    ) {
      return camera(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        _time0,
        _time1
      );
    }
};

color ray_color(const ray& r, const color& background, const hittable& world, int depth) {
  hit_record rec;
  if (depth <= 0) {
    return color(0, 0, 0);
  }

  if (!world.hit(r, 0.001, infinity, rec)) {
    return background;
  }
  ray scattered;
  color attenuation;
  color emitted = rec.mat_ptr->emitted(rec.u, rec.v, rec.p);

  if (!rec.mat_ptr->scatter(r, rec, attenuation, scattered)) {
    return emitted;
  }

  return emitted + attenuation * ray_color(scattered, background, world, depth-1);
}

hittable_list two_spheres() {
  hittable_list objects;

  auto checker = make_shared<checker_texture>(color(0.2, 0.3, 0.1), color(0.9, 0.9, 0.9));
  objects.add(make_shared<sphere>(point3(0,-10,0), 10, make_shared<lambertian>(checker)));
  objects.add(make_shared<sphere>(point3(0, 10,0), 10, make_shared<lambertian>(checker)));

  return objects;
}

hittable_list random_scene() {
  hittable_list world;

  // shared_ptr<material> ground_material = make_shared<lambertian>(color(0.5, 0.5, 0.5));
  // world.add(make_shared<sphere>(point3(0,-1000,0), 1000, ground_material));
  auto checker = make_shared<checker_texture>(color(0.2, 0.3, 0.1), color(0.9, 0.9, 0.9));
  world.add(make_shared<sphere>(point3(0,-100,0), 100, make_shared<lambertian>(checker)));

  for (int a = -3; a < 3; a++) {
    for (int b = -3; b < 3; b++) {
      auto choose_mat = random_double();
      point3 center(a + 0.9 * random_double(), 0.2, b + 0.9 * random_double());

      if ((center - point3(4, 0.2, 0)).length() > 0.9) {
        shared_ptr<material> sphere_material;

        if (choose_mat < 0.8) {
          vec3 albedo = color::random() * color::random();
          sphere_material = make_shared<lambertian>(albedo);
          auto center2 = center + vec3(0, random_double(0, 0.5), 0);
          world.add(make_shared<moving_sphere>(center, center2, 0.0, 1.0, 0.2, sphere_material));
        } else if (choose_mat < 0.95) {
          vec3 albedo = color::random(0.5, 1);
          double fuzz = random_double(0, 0.5);
          sphere_material = make_shared<metal>(albedo, fuzz);
          world.add(make_shared<sphere>(center, 0.2, sphere_material));
        } else {
          sphere_material = make_shared<dielectric>(1.5);
          world.add(make_shared<sphere>(center, 0.2, sphere_material));
        }
      }
    }
  }

  auto material1 = make_shared<dielectric>(1.5);
  world.add(make_shared<sphere>(point3(0, 1, 0), 0.3, material1));

  auto material2 = make_shared<lambertian>(color(0.4, 0.2, 0.1));
  world.add(make_shared<sphere>(point3(-2, 1, 0), 0.3, material2));

  auto material3 = make_shared<metal>(color(0.7, 0.6, 0.5), 0.0);
  world.add(make_shared<sphere>(point3(2, 1, 0), 0.3, material3));

  return world;
}

hittable_list two_perlin_spheres() {
  hittable_list objects;

  auto pertext = make_shared<noise_texture>(4);
  objects.add(make_shared<sphere>(point3(0,-100,0), 100, make_shared<lambertian>(pertext)));
  objects.add(make_shared<sphere>(point3(0, 1, 0), 1, make_shared<lambertian>(pertext)));

  return objects;
}

hittable_list earth() {
  auto earth_texture = make_shared<image_texture>("earthmap.jpeg");
  auto earth_surface = make_shared<lambertian>(earth_texture);
  auto globe = make_shared<sphere>(point3(0, 0, 0), 2, earth_surface);

  return hittable_list(globe);
}

hittable_list simple_light() {
    hittable_list objects;

    auto pertext = make_shared<noise_texture>(4);
    objects.add(make_shared<sphere>(point3(0,-1000,0), 1000, make_shared<lambertian>(pertext)));
    objects.add(make_shared<sphere>(point3(0,2,0), 2, make_shared<lambertian>(pertext)));

    auto difflight = make_shared<diffuse_light>(color(4,4,4));
    objects.add(make_shared<xy_rect>(3, 5, 1, 3, -2, difflight));

    return objects;
}

hittable_list cornell_box() {
  hittable_list objects;

  auto red   = make_shared<lambertian>(color(.65, .05, .05));
  auto white = make_shared<lambertian>(color(.73, .73, .73));
  auto green = make_shared<lambertian>(color(.12, .45, .15));
  auto light = make_shared<diffuse_light>(color(7, 7, 7));

  objects.add(make_shared<yz_rect>(0, 555, 0, 555, 555, green));
  objects.add(make_shared<yz_rect>(0, 555, 0, 555, 0, red));
  objects.add(make_shared<xz_rect>(113, 443, 127, 432, 554, light));
  objects.add(make_shared<xz_rect>(0, 555, 0, 555, 555, white));
  objects.add(make_shared<xz_rect>(0, 555, 0, 555, 0, white));
  objects.add(make_shared<xy_rect>(0, 555, 0, 555, 555, white));

  shared_ptr<hittable> box1 = make_shared<box>(point3(0,0,0), point3(165,330,165), white);
  box1 = make_shared<rotate_y>(box1, 15);
  box1 = make_shared<translate>(box1, vec3(265,0,295));

  shared_ptr<hittable> box2 = make_shared<box>(point3(0,0,0), point3(165,165,165), white);
  box2 = make_shared<rotate_y>(box2, -18);
  box2 = make_shared<translate>(box2, vec3(130,0,65));

  return objects;
}

hittable_list cornell_smoke() {
  hittable_list objects;

  auto red   = make_shared<lambertian>(color(.65, .05, .05));
  auto white = make_shared<lambertian>(color(.73, .73, .73));
  auto green = make_shared<lambertian>(color(.12, .45, .15));
  auto light = make_shared<diffuse_light>(color(7, 7, 7));

  objects.add(make_shared<yz_rect>(0, 555, 0, 555, 555, green));
  objects.add(make_shared<yz_rect>(0, 555, 0, 555, 0, red));
  objects.add(make_shared<xz_rect>(113, 443, 127, 432, 554, light));
  objects.add(make_shared<xz_rect>(0, 555, 0, 555, 555, white));
  objects.add(make_shared<xz_rect>(0, 555, 0, 555, 0, white));
  objects.add(make_shared<xy_rect>(0, 555, 0, 555, 555, white));

  shared_ptr<hittable> box1 = make_shared<box>(point3(0,0,0), point3(165,330,165), white);
  box1 = make_shared<rotate_y>(box1, 15);
  box1 = make_shared<translate>(box1, vec3(265,0,295));

  shared_ptr<hittable> box2 = make_shared<box>(point3(0,0,0), point3(165,165,165), white);
  box2 = make_shared<rotate_y>(box2, -18);
  box2 = make_shared<translate>(box2, vec3(130,0,65));

  objects.add(make_shared<constant_medium>(box1, 0.01, color(0,0,0)));
  objects.add(make_shared<constant_medium>(box2, 0.01, color(1,1,1)));

  return objects;
}

#endif
