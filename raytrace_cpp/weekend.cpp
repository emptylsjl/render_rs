
#include <iostream>
#include <string>
#include <utility>
#include <vector>
#include <limits>

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"

#include <Eigen/Dense>
using namespace Eigen;

using v2d = Vector2d;
using v3d = Vector3d;
using v4d = Vector4d;
using rgbV3d = Vector3d;
using rgbV4d = Vector4d;

class ray {
public:
    ray(v3d  origin, v3d  direction) : pt(std::move(origin)), dir(std::move(direction)){}

    v3d at(double t) const {return pt + t * dir;}

public:
    v3d pt;
    v3d dir;
};

class rgba {
public:
    rgba() : c{0, 0, 0, 0} {}
    rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a) : c{r, g, b, a} {}
    rgba(v3d c) : c{uint8_t(c[0] * 255), uint8_t(c[1] * 255), uint8_t(c[2] * 255), 255 } {}
    rgba(v4d c) : c{uint8_t(c[0] * 255), uint8_t(c[1] * 255), uint8_t(c[2] * 255), uint8_t(c[3] * 255) } {}

    uint8_t r() { return c[3]; }
    uint8_t g() { return c[2]; }
    uint8_t b() { return c[1]; }
    uint8_t a() { return c[0]; }
    uint8_t* rgba_slice() { return c; }
    uint32_t rgba_u32() { return *(uint32_t*)c; }

public:
    uint8_t c[4];
};

class sphere {
public:
    sphere() {}
    sphere(v3d cpt, double r) : cpt(std::move(cpt)), r(r) {};
public:
    v3d cpt;
    double r;
};

struct spt {
    v3d p;
    v3d n;
    double t;
};

struct hitSpt {
    spt pts[2];
    int min;
    int count;
};

struct hit_record {
    v3d p;
};

const auto aspect_ratio = 16.0 / 8.0;
const int W = 800;
const int H = W/aspect_ratio;

void write_to_img(uint8_t *img, int x, int y, rgba color) {
    int pos = 4 * (x + y*W);
    *(uint32_t*)(img+pos) = color.rgba_u32();
}

hitSpt on_hit(const v3d& sphere, double sphere_r, const ray& r) {
    v3d ps = r.pt - sphere;
    auto a = r.dir.dot(r.dir);
    auto b = r.dir.dot(ps) * 2;
    auto c = ps.dot(ps) - sphere_r*sphere_r;
    double d = b*b-4*a*c;
    if (d < 0) return { {}, {}, 0};
    auto t0 = (-b-sqrt(d))/2*a;
    auto t1 = (-b+sqrt(d))/2*a;
    spt a0 = {r.at(t0), (r.at(t0)-sphere)/sphere_r, t0};
    spt a1 = {r.at(t1), (r.at(t1)-sphere)/sphere_r, t1};
    return { {a0, a1}, a1.t < a0.t, (a1.t != a0.t) + 1 };
}

hitSpt on_hit2(const v3d& sphere, double sphere_r, const ray& r) {
    v3d ps = r.pt - sphere;
    auto a = r.dir.dot(r.dir);
    auto b = r.dir.dot(ps);
    auto c = ps.dot(ps) - sphere_r*sphere_r;
    double d = b*b-a*c;
    if (d < 0) return { {}, {}, 0};
    auto t0 = (-b-sqrt(d))/a;
    auto t1 = (-b+sqrt(d))/a;
    spt a0 = {r.at(t0), (r.at(t0)-sphere)/sphere_r, t0};
    spt a1 = {r.at(t1), (r.at(t1)-sphere)/sphere_r, t1};
    return { {a0, a1}, a1.t > a0.t, (a1.t != a0.t) + 1 };
}

rgbV3d ray_color(const ray& r) {
    hitSpt hpts = on_hit2(v3d(0, 0, -1), 0.6, r);
    if (hpts.count > 0) {
        v3d N = (r.at(hpts.pts[0].t) - v3d(0, 0, -1)).normalized();
        return 0.5*v3d(N.x()+1, N.y()+1, N.z()+1);
    }
    auto t = 0.5*(r.dir.normalized().y() + 1.0);
    return (1.0-t)*rgbV3d(1.0, 1.0, 1.0) + t*rgbV3d(0.5, 0.7, 1.0);
}

void drawA() {
    const std::string filename("D:\\cur\\render_rs\\raytrace_cpp\\img\\raytrace.png");
    auto *image = (uint8_t*)calloc(H * W * 4, sizeof(uint8_t));

    auto viewport_height = 2.0;
    auto viewport_width = aspect_ratio * viewport_height;
    auto focal_length = 1.0;
    auto origin = v3d(0, 0, 0);
    auto horizontal = v3d(viewport_width, 0, 0);
    auto vertical = v3d(0, viewport_height, 0);
    auto lower_left_corner = origin - horizontal/2 - vertical/2 - v3d(0, 0, focal_length);

    for (int i = 0, ii = W-1; i < W; ++i, --ii) {
        for (int j = 0, jj = H-1; j < H; ++j, --jj) {
//            printf("%d ", jj);
            auto u = double(i) / W;
            auto v = double(j) / H;
            ray r(origin, lower_left_corner + u*horizontal + v*vertical);
            v3d color = ray_color(r);
            write_to_img(image, i, jj, rgba(color));

        }
    }
//    std::cout << image << std::endl;
    stbi_write_png(filename.c_str(), W, H, 4, image, W*4);
    printf("\n\n233\n");


}


int main() {
    drawA();
    return 0;
}
