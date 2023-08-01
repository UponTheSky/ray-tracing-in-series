#ifndef TEXTURE_H
#define TEXTURE_H

#include "perlin.h"

class texture {
 public:
  virtual color value(double u, double v, const point3& p) const = 0;
};

class solid_color : public texture {
 public:
  solid_color() {}
  solid_color(color c) : color_value(c) {}
  solid_color(double red, double green, double blue)
      : solid_color(color(red, green, blue)) {}

  virtual color value(double u, double v, const point3& p) const override {
    return color_value;
  }

 private:
  color color_value;
};

class checker_texture : public texture {
 public:
  checker_texture() {}
  checker_texture(shared_ptr<texture> _even, shared_ptr<texture> _odd)
      : even(_even), odd(_odd) {}
  checker_texture(color c1, color c2)
      : even(make_shared<solid_color>(c1)), odd(make_shared<solid_color>(c2)) {}

  virtual color value(double u, double v, const point3& p) const override {
    auto sines = sin(10 * p.x()) * sin(10 * p.y()) * sin(10 * p.z());
    return sines < 0 ? odd->value(u, v, p) : even->value(u, v, p);
  }

 private:
  shared_ptr<texture> odd;
  shared_ptr<texture> even;
};

class noise_texture : public texture {
 public:
  noise_texture() {}
  noise_texture(double sc) : scale(sc) {}

  virtual color value(double u, double v, const point3& p) const override {
    // return color(1, 1, 1) * 0.5 * (1.0 + noise.noise(scale * p));
    // return color(1, 1, 1) * noise.turb(scale * p); // with turbulence
    return color(1, 1, 1) * 0.5 *
        (1 +
         sin(scale * p.z() +
             10 * noise.turb(p))); // make color proportional to sine
  }

 private:
  perlin noise;
  double scale;
};

// class image_texture : public texture {
//   public:
//     static const int bytes_per_pixel = 3;

//     image_texture() : data(nullptr), width(0), height(0),
//     bytes_per_scanline(0) {} image_texture(const char* filename) {
//       int components_per_pixel = bytes_per_pixel;
//       data = stbi_load(
//         filename, &width, &height, &components_per_pixel,
//         components_per_pixel);

//       if (!data) {
//         std::cerr << "ERROR: Could not load texture image file " << filename
//         << ".\n"; width = height = 0;
//       }

//       bytes_per_scanline = bytes_per_pixel * width;
//     }

//     ~image_texture() {
//       delete data;
//     }

//     virtual color value(double u, double v, const vec3& p) const override {
//       if (data == nullptr) {
//         return color(0, 1, 1);
//       }

//       u = clamp(u, 0, 1);
//       v = 1 - clamp(v, 0, 1);

//       int i = static_cast<int>(u * width);
//       int j = static_cast<int>(v * height);

//       if (i >= width) {
//         i = width - 1;
//       }

//       if (j >= height) {
//         j = height - 1;
//       }

//       const double color_scale = 1 / 255.0;
//       unsigned char* pixel = data + j * bytes_per_scanline + i *
//       bytes_per_pixel;

//       return color(color_scale * pixel[0], color_scale * pixel[1],
//       color_scale * pixel[2]);
//     }

//   private:
//     unsigned char *data;
//     int width, height;
//     int bytes_per_scanline;
// };

#endif
