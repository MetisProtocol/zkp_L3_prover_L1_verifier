#include <new>
#include "some_object.hpp"

extern "C" void init(void* preallocated_memory, int x, int y)
{
  new(preallocated_memory) Rectangle(x, y);
}

extern "C" int volume(Rectangle* rectangle, int depth)
{
  return rectangle->area() * depth;
}
