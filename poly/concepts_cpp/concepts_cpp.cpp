#include <iostream>

class IDemo
{
public:
   virtual ~IDemo() {}
   virtual uint32_t Some_Operation(uint32_t a, uint32_t b) = 0;
};


class Child_A : public IDemo
{
public:
   uint32_t Some_Operation(uint32_t a, uint32_t b) {
      return 0;
   }
};

class Child_B : public IDemo
{
public:
   uint32_t Some_Operation(uint32_t a, uint32_t b) {
      return 1;
   }
};

uint32_t better_example(IDemo& demo)
{
   return demo.Some_Operation(1, 2);
}

class Container {

public:
   Container(IDemo& demo) : value_m(demo) {}
   IDemo& value_m;
};


int main()
{
   Child_B child_b;
   uint32_t product = better_example(child_b);

   Child_A child_a;
   uint32_t sum = better_example(child_a);

   auto container = Container(child_a);
   container.value_m.Some_Operation(1, 2);
}