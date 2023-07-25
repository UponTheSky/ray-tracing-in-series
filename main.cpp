#include <iostream>

#include "sphere.h"
#include "hittable_list.h"
#include "moving_sphere.h"
#include "color.h"
#include "rtweekend.h"
#include "camera.h"
#include "texture.h"

color ray_color(const ray& r, const hittable& world, int depth) {
  hit_record rec;
  if (depth <= 0) {
    return color(0, 0, 0);
  }

  if (world.hit(r, 0.001, infinity, rec)) {
    ray scattered;
    color attenuation;
    if (rec.mat_ptr->scatter(r, rec, attenuation, scattered)) {
      return attenuation * ray_color(scattered, world, depth-1); // reflection; recursive
    }
    return color(0, 0, 0);
  }
  vec3 unit_direction = unit_vector(r.direction());
  auto t = 0.5 * (unit_direction.y() + 1.0); // rescale to [0, 1]
  return (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0);
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

  auto pertext = make_shared<noise_texture>();
  objects.add(make_shared<sphere>(point3(0,-5,0), 5, make_shared<lambertian>(pertext)));
  objects.add(make_shared<sphere>(point3(0, 1, 0), 1, make_shared<lambertian>(pertext)));

  return objects;
}

int main() {
  // image
  const double aspect_ratio = 16.0 / 9.0;
  const int image_width = 400;
  const int samples_per_pixel = 10;
  const int max_depth = 10;

  hittable_list world;

  point3 lookfrom;
  point3 lookat;
  auto vfov = 40.0;
  auto aperture = 0.0;

  switch (0) {
      case 1:
          world = random_scene();
          lookfrom = point3(13,2,3);
          lookat = point3(0,0,0);
          vfov = 20.0;
          aperture = 0.1;
          break;

      case 2:
          world = two_spheres();
          lookfrom = point3(13,2,3);
          lookat = point3(0,0,0);
          vfov = 20.0;
          break;
      default:
      case 3:
          world = two_perlin_spheres();
          lookfrom = point3(13,2,3);
          lookat = point3(0,0,0);
          vfov = 20.0;
          break;
  }

  // Camera
  vec3 vup(0,1,0);
  auto dist_to_focus = 10.0;
  int image_height = static_cast<int>(image_width / aspect_ratio);

  camera cam(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

  // render

  std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

  for (int j = image_height - 1; j >= 0; j--) {
    std::cerr << "\rScanlines remaining: " << j << ' ' << std::flush;
    for (int i = 0; i < image_width; i++) {
      color pixel_color(0, 0, 0);
      for (int s = 0; s < samples_per_pixel; ++s) {
        double u = double(i + random_double()) / (image_width - 1);
        double v = double(j + random_double()) / (image_height - 1);
        ray r = cam.get_ray(u, v);
        pixel_color += ray_color(r, world, max_depth);
      }
      write_color(std::cout, pixel_color, samples_per_pixel);
    }
  }
  std::cerr << "\nDone.\n";
}
