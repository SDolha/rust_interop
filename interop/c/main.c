#include <stdio.h>

typedef struct {
    char flag;
    char* data;
} CObject;

extern CObject c_update(CObject object);

int main() {
    CObject original;
    original.flag = 0;
    original.data = "Object Name=\"C💻\"";
    CObject updated = c_update(original);
    printf("flag: %d\n", updated.flag);
    printf("%s\n", updated.data);
    return 0;
}
