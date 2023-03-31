
#include <iostream>
#include <string>
#include <utility>
#include <vector>
#include <limits>

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"
#include "weekend.h"

#include <Eigen/Dense>
using namespace Eigen;

#include <chrono>
#include <memory>
#include <thread>

using namespace std::chrono;

using v2d = Vector2d;
//using v3d = Vector3d;
using v4d = Vector4d;
using rgbV3d = Vector3d;
using rgbV4d = Vector4d;

using v3d = vec3;


const double inf = std::numeric_limits<double>::infinity();
const double pi = 3.1415926535897932385;
const auto aspect_ratio = 16.0 / 8.0;
//const int W = 800;
const int W = 40;
const int H = W/aspect_ratio;
const double sample_count = 100;
const int max_depth = 50;
int count0 = 0;
int count1 = 0;
int count2 = 0;

double process(double c) {
    return c;
//    return sqrt(c / sample_count);
}

uint8_t d2u8(double c) {
    return uint8_t(std::clamp(process(c), 0., 0.99999) * 255);
}

class rgba {
public:
    rgba() : c{0, 0, 0, 0} {}
    rgba(uint8_t r, uint8_t g, uint8_t b, uint8_t a) : c{r, g, b, a} {}
    explicit rgba(v3d c) : c{d2u8(c[0]), d2u8(c[1]), d2u8(c[2]), 255 } {}
    explicit rgba(v4d c) : c{uint8_t(c[0] * 255), uint8_t(c[1] * 255), uint8_t(c[2] * 255), uint8_t(c[3] * 255) } {}

    uint32_t rgba_u32() { return *(uint32_t*)c; }

public:
    uint8_t c[4];
};

class ray {
public:
    ray() {};
    ray(v3d origin, v3d dir) : pt(std::move(origin)), dir(std::move(dir)) {}

    v3d at(double t) const {return pt + t * dir;}

public:
    v3d pt;
    v3d dir;
};

struct hpt {
    v3d p;
    v3d n;
    v3d d;
    v3d a;
    double t;
    int hit;
};

class surface {
    public:
        virtual hpt dir(ray r, hpt hpt) = 0;
};

class rough : public surface {
    public:
        rough() {}
        rough(vec3 albedo) : albedo(albedo) {}

        virtual hpt dir(ray r, hpt hpt) {
            hpt.a = albedo;
            hpt.d = hpt.n + random_unit_vector();
            return hpt;
        }

    public:
        v3d albedo;
};

class metal : public surface {
    public:
        metal() {}
        metal(vec3 albedo) : albedo(albedo) {}

        virtual hpt dir(ray r, hpt hpt) {
            hpt.a = albedo;
            hpt.hit = (reflect(normalized(r.dir), hpt.n, hpt.d)) + 1;
            return hpt;
        }

    public:
        v3d albedo;
};

class sphere {
public:
    sphere() {}
//    sphere(double r, v3d cpt) : cpt(cpt), r(r), type(1) {};
    sphere(double r, v3d c, surface *s) : r(r), c(c), s(s) {};


public:
    v3d c;
    double r;
    surface *s;
};

class camera {
public:
    camera() {
        auto viewport_width = 4.;
        auto viewport_height = viewport_width/aspect_ratio;
        auto focal_length = 1.;

        origin = v3d(0, 0, 0);
        horizontal = v3d(viewport_width, 0.0, 0.0);
        vertical = v3d(0.0, viewport_height, 0.0);
        lower_left_corner = origin - horizontal/2 - vertical/2 - v3d(0, 0, focal_length);
//        std::cout << horizontal << std::endl;
//        std::cout << vertical << std::endl;
//        std::cout << lower_left_corner << std::endl;
    }

    ray get_ray(double u, double v) const {
        std::cout << lower_left_corner + u*horizontal + v*vertical - origin << std::endl;
        return ray(origin, lower_left_corner + u*horizontal + v*vertical - origin);
    }

private:
    v3d origin;
    v3d lower_left_corner;
    v3d horizontal;
    v3d vertical;

};

class world {
    public:
        world() {}
        world(sphere* s, int len) { objs.insert(objs.begin(), s, s+len); }

        void add(sphere s) { objs.push_back(s); }
        void clean() { objs.clear(); }
        uint32_t len() { return objs.size(); }

    hpt onHit(const ray& r) {

            hpt pt = {{}, {}, {}, {}, inf, 0};
            for (const auto& s : objs) {
                v3d ps = r.pt - s.c;
                double a = dot(r.dir, r.dir);
                double b = dot(r.dir, ps);
                double c = dot(ps, ps) - s.r*s.r;
                double d = b*b-a*c;
                if (d > 0) {
                    double t = std::min((-b-sqrt(d))/a, (-b+sqrt(d))/a);
                    if (t > 0.001 & t < pt.t) {
                        pt = s.s->dir(r, {r.at(t), (r.at(t)-s.c)/s.r, {}, {}, t, 1});
                    }
                }
            }
            return pt;
        }

    public:
        std::vector<sphere> objs;
};


bool sum(int *l, int len) {
    int a = 0;
    for (int i = 0; i < len; ++i)
        a += l[i];
    printf("\r%.02f %%", double(a)*100/W);
    std::flush(std::cout);
    return a < W;
}

void write_to_img(uint8_t *img, int x, int y, rgba color) {
    int pos = 4 * (x + y*W);
    *(uint32_t*)(img+pos) = color.rgba_u32();
}

v3d ray_color(world& w, ray r, int depth) {
    if (depth <= 0)
        return {0,0,0};

    auto [p, n, d, a, t, hit] = w.onHit(r);
//    if (hit == 2)
//        return {0,0,0};
    if (hit == 1) {
        count0++;
//        return n;
        return a * ray_color(w, {p, d}, depth-1);
    } else {
        count1++;
    }
    auto tp = 0.5*(normalized(r.dir).y() + 1.0);
    return (1.0-tp)*v3d(1.0, 1.0, 1.0) + tp*v3d(0.5, 0.7, 1.0);
}

int main() {
    const std::string filename("D:\\cur\\render_rs\\raytrace_cpp\\book_impl\\img\\raytrace.png");
    auto *image = (uint8_t*)calloc(H * W * 4, sizeof(uint8_t));
    setbuf(stdout, 0);


    auto a = rough(vec3(0.7, 0.7, 0.7));
    auto b = rough(vec3(0.9, 0.9, 0.9));
    auto c = rough(vec3(0.5, 0.5, 0.5));
//    metal(vec3(0.2, 0.6, 0.2));

    world w;
    w.add(sphere(0.5, v3d(0, 0, -1), &b));
    w.add(sphere(100, v3d(0,-100.5,-1), &a));
    w.add(sphere(0.5, v3d(-1.0, 0.0, -1.0), &c));

    camera cam;

    const int ct = 8;
    int rg = W / ct;
    int stage[ct] = {};
    std::thread th[ct] = {};

    auto f = [&](int pos, int st, int ed) {
        for (int i = st, ii = ed - 1; i < ed; ++i, --ii) {
            for (int j = 0, jj = H - 1; j < H; ++j, --jj) {
                v3d c;
                for (int k = 0; k < sample_count; ++k) {
                    auto u = (i + random_double()) / (W - 1);
                    auto v = (j + random_double()) / (H - 1);
                    c += ray_color(w, cam.get_ray(u, v), max_depth);
                }
                write_to_img(image, i, jj, rgba(c));
            }
            stage[pos] = i-st+1;
        }
    };


    auto st = high_resolution_clock::now();

    for (int i = 0; i < ct; ++i)
        th[i] = std::thread(f, i, i*rg, i*rg+rg);
    while (sum(stage, ct))
        std::this_thread::sleep_for(milliseconds(100));
    for (auto & t : th)
        t.join();

//    for (int i = 0, ii = W-1; i < W; ++i, --ii) {
//        for (int j = 0, jj = H-1; j < H; ++j, --jj) {
//            v3d c;
//            for (int k = 0; k < 1; ++k) {
////                auto u = (i + random_double()) / (W - 1);
////                auto v = (j + random_double()) / (H - 1);
//                auto u = double(i) / (W);
//                auto v = double(j) / (H);
//                c += ray_color(w, cam.get_ray(u, v), max_depth);
//            }
//            write_to_img(image, i, jj, rgba(c));
//        }
//        std::cout << i << " " << std::flush;
//    }

    duration<float> duration = (high_resolution_clock::now() - st);
    std::cout << "\nelapse: " << duration.count() << std::endl;

    printf("%d, %d\n", count0, count1);

    stbi_write_png(filename.c_str(), W, H, 4, image, W*4);
    printf("\n\n233\n");
}
