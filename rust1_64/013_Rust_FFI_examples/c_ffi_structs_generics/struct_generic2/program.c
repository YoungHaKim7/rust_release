#include "mylib.h"

int main(int argc, char *argv[]) {
  
  MyValue *mv = myvalue_create();
  myvalue_print(mv);
  myvalue_free(mv);
  
  return 0;

}
