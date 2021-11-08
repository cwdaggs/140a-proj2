float Foo(int val);
int Boo(int val);
unsigned long Moo(int val);

void main() {
  int x;
  x = Foo(Boo(Moo(8.0)));
}
