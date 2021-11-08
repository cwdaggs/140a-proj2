int x = 5;
float Foo(int val);
float y = -24;

void main() {
  int z = 50;

  Foo(7);
  float Foo(int val) {
    val = z + 2;
  }
}
